pub const ENTITY_REF_POST_ACTION: &'static str = r#"
#[derive(Clone, Copy)]
pub struct EntityRefPostAction<'a, 'b> {
    id: EntityId,
    ecs_post_action: EcsPostAction<'a, 'b>,
}

impl<'a, 'b> EntityRefPostAction<'a, 'b> {

    pub fn id(&self) -> EntityId { self.id }
    pub fn ecs(&self) -> EcsPostAction<'a, 'b> { self.ecs_post_action }

{{#each data_components}}
    pub fn {{name}}(&self) -> Option<&{{type}}> {
        self.ecs_post_action.get_{{name}}(self.id())
    }
    pub fn contains_{{name}}(&self) -> bool {
        self.ecs_post_action.contains_{{name}}(self.id())
    }
    {{#if copy}}
    pub fn copy_{{name}}(&self) -> Option<{{type}}> {
        self.{{name}}().map(|c| *c)
    }
    {{/if}}
{{/each}}
{{#each cell_components}}
    pub fn bare_{{name}}(&self) -> Option<&RefCell<{{type}}>> {
        self.ecs_post_action.bare_get_{{name}}(self.id())
    }
    pub fn borrow_{{name}}(&self) -> Option<Ref<{{type}}>> {
        self.ecs_post_action.borrow_{{name}}(self.id())
    }
    pub fn borrow_mut_{{name}}(&self) -> Option<RefMut<{{type}}>> {
        self.ecs_post_action.borrow_mut_{{name}}(self.id())
    }
    pub fn contains_{{name}}(&self) -> bool {
        self.ecs_post_action.contains_{{name}}(self.id())
    }
    pub fn {{name}}(&self) -> Option<&RefCell<{{type}}>> {
        self.bare_{{name}}()
    }
{{/each}}
{{#each flag_components}}
    pub fn contains_{{name}}(&self) -> bool {
        self.ecs_post_action.contains_{{name}}(self.id())
    }
    pub fn {{name}}(&self) -> bool {
        self.contains_{{name}}()
    }
{{/each}}
}
"#;