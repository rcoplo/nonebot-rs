use crate::log::colored::*;
use std::collections::HashMap;
use std::sync::Arc;

/// 定时任务 trait

pub trait ScheduledJob {
    /// 定时任务标识
    fn name(&self) -> &'static str;
    fn cron(&self) -> &'static str;
    fn call(&self, bot: Arc<crate::Bot>) -> std::pin::Pin<Box<dyn std::future::Future<Output=()> + Send + 'static>>;
}

/// 定时任务执行器
#[derive(Clone)]
pub struct Scheduler {
    inner: tokio_cron_scheduler::JobScheduler,
    bots: HashMap<i64, Vec<uuid::Uuid>>,
    jobs: Vec<Arc<Box<dyn ScheduledJob + Sync + Send + 'static>>>,
    config: SchedulerConfig,
}

impl std::fmt::Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scheduler")
            .field("config", &self.config)
            .finish()
    }
}
/// Scheduler Plugin Config struct
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SchedulerConfig {
    #[serde(default)]
    disable: bool,
    #[serde(flatten)]
    jobs: HashMap<String, JobConfig>,
}

/// Config for each Job
#[derive(Debug, Clone, serde::Deserialize)]
pub struct JobConfig {
    #[serde(flatten)]
    custom: HashMap<String, toml::Value>,
}

impl Scheduler {
    pub async fn new() -> Self {
        Self {
            inner: tokio_cron_scheduler::JobScheduler::new().await.expect("JobScheduler start failed"),
            bots: HashMap::new(),
            jobs: vec![],
            config: SchedulerConfig { disable: false, jobs: HashMap::new() },
        }
    }

    /// 向定时任务执行器中添加一个定时任务
    pub async fn add_task(&mut self, job: Box<dyn ScheduledJob + Sync + Send + 'static>) {
        let job = Arc::new(job);
        self.jobs.push(job);
    }
    async fn run(mut self, mut event_receiver: crate::EventReceiver) {
        while let Ok(event) = event_receiver.recv().await {
            match event {
                crate::event::Event::Nonebot(bot) => {
                    match bot {
                        crate::event::NbEvent::BotConnect { bot } => {
                            let arc = Arc::new(bot);
                            let mut vec = vec![];
                            for job in &self.jobs {
                                let bot = Arc::clone(&arc);
                                let _job = job.clone();
                                crate::log::event!(
                                   crate::log::Level::INFO,
                                   "Bot [{}] Job creator created -> Job NAME: {}",
                                   bot.bot_id.to_string().red(),
                                   _job.name().blue()
                               );
                                let job = tokio_cron_scheduler::Job::new_async(_job.cron(), move |_, _| {
                                    _job.call(bot.clone())
                                }).unwrap();
                                vec.push(job.guid());
                                self.inner.add(job).await.expect("job add failed");
                            }
                            self.bots.insert(arc.bot_id, vec);
                        }
                        crate::event::NbEvent::BotDisconnect { bot } => {
                            if let Some(uuid) = self.bots.get(&bot.bot_id) {
                                for u in uuid {
                                    self.inner.remove(u).await.expect("job remove failed");
                                }
                                crate::log::event!(
                                   crate::log::Level::INFO,
                                   "Bot [{}] Job created deleted",
                                   bot.bot_id.to_string().red(),
                               );
                                self.bots.remove(&bot.bot_id);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        self.start().await;
    }
    async fn start(&self) {
        self.inner.start().await.unwrap();
    }
}

pub trait ArcScheduledJob {
    fn name(&self) -> &'static str;
    fn cron(&self) -> &'static str;
    fn call(self: &Arc<Self>, bot: Arc<crate::Bot>) -> std::pin::Pin<Box<dyn std::future::Future<Output=()> + Send + 'static>>;
}

impl<T: ArcScheduledJob> ScheduledJob for Arc<T> {
    fn name(&self) -> &'static str { <T as ArcScheduledJob>::name(&self) }
    fn cron(&self) -> &'static str {
        <T as ArcScheduledJob>::cron(&self)
    }
    fn call(&self, bot: Arc<crate::Bot>) -> std::pin::Pin<Box<dyn std::future::Future<Output=()> + Send + 'static>> {
        <T as ArcScheduledJob>::call(&self, bot)
    }
}


#[async_trait::async_trait]
impl crate::Plugin for Scheduler {
    fn run(&self, event_receiver: crate::EventReceiver, _: crate::BotGetter) {
        let job = self.clone();
        if !job.config.disable {
            tokio::spawn(job.run(event_receiver));
        }
    }

    fn plugin_name(&self) -> &'static str {
        "Scheduler"
    }

    async fn load_config(&mut self, config: toml::Value) {
        self.config = config.try_into().expect("Scheduler load config fail");
        crate::log::event!(
            crate::log::Level::INFO,
            "[{}] Loaded config {:?}",
            self.plugin_name().red(),
            self.config
        );
    }
}