use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_node::GritTargetSyntaxKind;
use anyhow::Result;
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::pattern::{
    CodeSnippet, DynamicPattern, Matcher, Pattern, PatternName, ResolvedPattern, State,
};
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub(crate) struct GritCodeSnippet {
    pub(crate) patterns: Vec<(GritTargetSyntaxKind, Pattern<GritQueryContext>)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<GritQueryContext>>,
}

impl CodeSnippet<GritQueryContext> for GritCodeSnippet {
    fn patterns(&self) -> impl Iterator<Item = &Pattern<GritQueryContext>> {
        TodoIterator { _snippet: self }
    }

    fn dynamic_snippet(&self) -> Option<&DynamicPattern<GritQueryContext>> {
        todo!()
    }
}

impl Matcher<GritQueryContext> for GritCodeSnippet {
    fn execute<'a>(
        &'a self,
        resolved: &GritResolvedPattern<'a>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let Some(binding) = resolved.get_last_binding() else {
            return Ok(resolved.text(&state.files, context.language())?.trim() == self.source);
        };

        let Some(node) = binding.singleton() else {
            return Ok(false);
        };

        if let Some((_, pattern)) = self.patterns.iter().find(|(kind, _)| *kind == node.kind()) {
            pattern.execute(resolved, state, context, logs)
        } else {
            Ok(false)
        }
    }
}

impl PatternName for GritCodeSnippet {
    fn name(&self) -> &'static str {
        "CodeSnippet"
    }
}

#[derive(Clone)]
struct TodoIterator<'a> {
    _snippet: &'a GritCodeSnippet,
}

impl<'a> Iterator for TodoIterator<'a> {
    type Item = &'a Pattern<GritQueryContext>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
