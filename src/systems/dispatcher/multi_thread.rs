use super::UnifiedDispatcher;
use specs::prelude::*;

pub macro construct_dispatcher(
    $(
        (
            $type:ident,
            $name:expr,
            $deps:expr
        )
    ),*
) {
    use specs::DispatcherBuilder;

    let dispatcher = DispatcherBuilder::new()
        $(
            .with($type{}, $name, $deps)
        )*
        .build();

    let dispatch = MultiThreadedDispatcher{
        dispatcher
    };

    return Box::new(dispatch);
}

pub struct MultiThreadedDispatcher {
    pub dispatcher: specs::Dispatcher<'static, 'static>,
}

impl UnifiedDispatcher for MultiThreadedDispatcher {
    fn run_now(&mut self, ecs: &mut World) {
        self.dispatcher.dispatch(ecs);
        // crate::effects::run_effects_queue(&mut *ecs);
    }

    fn setup(&mut self, ecs: &mut World) {
        self.dispatcher.setup(ecs);
    }
}
