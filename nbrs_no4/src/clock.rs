use nonebot_rs::scheduler::prelude::*;

#[allow(dead_code)]
pub fn clock(nb: &nonebot_rs::Nonebot) -> Job {
    let bot_getter = nb.bot_getter.clone();
    Job::new_async("1 * * * * *", move |_, _| Box::pin({
        let bots = bot_getter.borrow().clone();
        for (_, bot) in bots {
            let bot = bot.clone();
            tokio::spawn(send_a_msg(bot));
        }
    }))
    .unwrap()
}

// Just for test
#[allow(dead_code)]
async fn send_a_msg(bot: nonebot_rs::Bot) {
    for superuser in &bot.config.superusers {
        bot.send_private_msg(
            superuser.parse::<i64>().expect("管理员账号只能是整数!"),
            vec![Message::text("One minute passed.".to_string())],
        )
        .await;
    }
}
