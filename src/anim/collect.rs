#[macro_export]
macro_rules! defn_anim {
    ($i:item) => {
        #[derive(
            Debug,
            Copy,
            Clone,
            Default,
            Reflect,
            PartialEq,
            Eq,
            Hash,
            bevy_2delight_macros::AnimStateMachine,
        )]
        $i
    };
}
