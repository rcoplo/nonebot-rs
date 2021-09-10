use nbrs_matcher_r6s::r6s;
use nonebot_rs;

mod clock;

fn main() {
    let mut nb = nonebot_rs::Nonebot::new();
    let config = nb.config.clone();
    nb.matchers
        .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb())
        .add_message_matcher(nonebot_rs::builtin::echo::echo2())
        .add_message_matcher(nonebot_rs::builtin::bot_status::bot_status(
            config.get_matcher_config("bot_status"),
        ))
        .add_message_matchers(r6s());
    // nb.scheduler.add(clock::clock(&nb)).unwrap();
    nb.run()
}
