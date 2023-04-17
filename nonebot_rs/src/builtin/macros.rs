/// 注册通配匹配器
///
/// 为 `Matcher` 注册一个匹配所有输入的 `match_` 函数
#[allow(unused_macros)]
#[macro_export]
macro_rules! on_match_all {
    () => {
        fn match_(&mut self, _: &mut MessageEvent) -> bool {
            true
        }
    };
}

/// 注册命令匹配器
///
/// 为 `Matcher` 注册一个命令匹配器，匹配的命令将从 `raw_message` 中移除
/// 可以同时接受多个字符串作为同一命令
#[allow(unused_macros)]
#[macro_export]
macro_rules! _on_command {
    ($event_type: ty, $command: expr) => {
        fn match_(&mut self, event: &mut $event_type) -> bool {
            if event.get_raw_message().starts_with($command) {
                event.set_raw_message(event.get_raw_message().replace($command, "").to_string());
                true
            } else {
                false
            }
        }
    };
    // fn match_(&self, event: &mut MessageEvent) -> bool {
       //     if event.get_raw_message().starts_with(r"echo ") {
       //         event.set_raw_message(event.get_raw_message().replace(r"echo ", "").to_string());
       //         true
       //     } else {
       //         false
       //     }
       // }
    ($event_type: ty, $($x:expr),*) => {
        fn match_(&mut self, event: &mut $event_type) -> bool {
            let mut commands:Vec<&str> = Vec::new();
            $(
                commands.push($x);
            )*
            for command in commands.iter() {
                if event.get_raw_message().starts_with(command) {
                    event.set_raw_message(event.get_raw_message().replace(command, "").to_string());
                    return true;
                }
            }
            false
        }
    };
}

/// 注册字符匹配器
///
/// 为 `Matcher` 注册一个字符匹配器，匹配字符将不会移除
/// 可以同时接受多个字符串
#[allow(unused_macros)]
#[macro_export]
macro_rules! on_start_with {
    ($event_type: ty, $command: expr) => {
        fn match_(&mut self, event: &mut $event_type) -> bool {
            if event.get_raw_message().starts_with($command) {
                true
            } else {
                false
            }
        }
    };
    ($event_type: ty, $($x:expr),*) => {
        fn match_(&mut self, event: &mut $event_type) -> bool {
            let mut commands:Vec<&str> = Vec::new();
            $(
                commands.push($x);
            )*
            for command in commands.iter() {
                if event.get_raw_message().starts_with(command) {
                    return true;
                }
            }
            false
        }
    };
}

#[doc(hidden)]
#[allow(unused_macros)]
#[macro_export]
macro_rules! matcher_request {
    ($b:block) => {
        #[derive(Clone)]
        struct Temp {}

        #[async_trait]
        impl Handler<MessageEvent> for Temp {
            on_match_all!();
            async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
                $b
            }
        }

        matcher
            .set_message_matcher(
                event.get_self_id(),
                build_temp_message_event_matcher(&event, Temp {}),
            )
            .await;
    }; // #[derive(Clone)]
       // struct Temp {}

       // #[async_trait]
       // impl Handler<MessageEvent> for Temp {
       //     on_match_all!();
       //     async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
       //         let msg = event.get_raw_message();
       //         matcher.send_text(&encode(&msg)).await;
       //     }
       // }

       // matcher
       //     .set_message_matcher(
       //         event.get_self_id(),
       //         build_temp_message_event_matcher(&event, Temp {}),
       //     )
       //     .await;
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! on_command {
     ($event_type: ty,$ident: ident, $($enum_name:ident => $command:expr),*,$b:block) => {
         on_command!($event_type, $ident{}, $($enum_name => $command),*,$b);
     };
    ($event_type: ty,$ident: ident{$($ident_:ident: $ident_ty:ty),*}, $($enum_name:ident => $command:expr),*,$b:block) => {

        #[derive(Clone,Default)]
        pub enum Commands {
            #[default]
            None,
            $(
            $enum_name(nonebot_rs::message::MessageChain,Vec<String>)
            ),*
        }

        impl Commands {

            fn _command(&mut self, event: &mut $event_type) -> (bool,Option<Self>) {
                $(
                   if event.get_raw_message().starts_with($command) {
                        let new_message = event.get_raw_message().replace($command, "");
                        event.set_raw_message(new_message.clone());
                        let vec = new_message.split_whitespace().map(|x|x.to_string()).collect();
                        let message = event.get_message_chain();
                        return (true,Some(Commands::$enum_name(message,vec)));
                   }
                )*
                (false,None)
            }
        }

        #[derive(Clone)]
        pub struct $ident {
            commands:Commands,
            $($ident_:$ident_ty),*
        }

        impl $ident {
            pub fn new() -> $ident {
                Self{
                    commands:Commands::default(),
                    $($ident_:Default::default()),*
                }
            }

            pub fn run() -> nonebot_rs::matcher::Matcher<$event_type>{
                $b
            }
        }
    };

}
/// match_
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! on_match {
    ($event_type: ty) => {
        fn match_(&mut self, event: &mut $event_type) -> bool {
            let (b,command) = self.commands._command(event);
            if let Some(c) = command{
                self.commands = c;
            }
            b
        }
    };
}

#[cfg(test)]
mod test {
    use tracing_subscriber::util::SubscriberInitExt;
    use crate::Message;

    #[derive(Clone)]
    pub struct TextPlugin {
        commands: Commands,
    }

    impl TextPlugin {
        fn _command(&mut self, event: &mut nonebot_rs::event::MessageEvent) -> bool {
            let (b, command) = self.commands._command(event);
            if let Some(c) = command {
                self.commands = c;
            }
            b
        }

        fn match_(&self, event: &mut nonebot_rs::event::MessageEvent) -> bool {
            let mut plugin = self.clone();
            if plugin._command(event) {
                return true;
            }
            false
        }
    }

    #[derive(Clone, Default)]
    pub enum Commands {
        A(crate::message::MessageChain, Vec<String>)
    }


    impl Commands {
        fn run(&self) -> nonebot_rs::matcher::Matcher<nonebot_rs::event::MessageEvent> {
            nonebot_rs::matcher::Matcher::new(stringify!(self), Self {})
                .add_pre_matcher(nonebot_rs::builtin::prematchers::to_me())
        }
        fn _command(&self, event: &mut crate::event::MessageEvent) -> (bool, Option<Commands>) {
            if event.get_raw_message().starts_with("e") {
                let new_message = event.get_raw_message().replace("e", "");
                event.set_raw_message(new_message.clone());
                let vec = new_message.split_whitespace().map(|x| x.to_string()).collect();
                return (true, Some(Commands::A(event.get_message_chain(), vec)));
            }
            (false, None)
        }
    }
}