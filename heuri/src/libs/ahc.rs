
/* StateTrait {{{ */
/// Trait for state representation.
#[allow(dead_code)]
trait StateTrait: Clone {

    type Action: Clone;
    type Score: Ord;


    fn legal_actions(&self) -> Vec<Self::Action>;
    fn forward(&mut self, action: &Self::Action);
    fn score(&self) -> Self::Score;
    fn is_done(&self) -> bool;
    fn dbg(&self);


    /* beam_search {{{ */
    fn beam_search(state: &mut Self, beam_width: usize, beam_depth: usize) -> Option<Self::Action> {
        let mut beam: Vec<(Self, Option<Self::Action>)> = Vec::new();
        let mut next_beam = Vec::new();

        beam.push((state.clone(), None));

        for t in 0..beam_depth {
            next_beam.clear();

            while let Some((state, first_action)) = beam.pop() {
                for action in state.legal_actions() {
                    let mut next_state = state.clone();
                    next_state.forward(&action);
                    if t == 0 {
                        next_beam.push((next_state, Some(action)));
                    } else {
                        next_beam.push((next_state, first_action.clone()));
                    }
                }
            }

            next_beam.sort_by_key(|(state, _)| std::cmp::Reverse(state.score()));
            next_beam.truncate(beam_width);

            std::mem::swap(&mut beam, &mut next_beam);

            match beam.first() {
                Some((state, _)) if state.is_done() => break,
                None => break,
                _ => (),
            }
        }

        let mut best_state = beam[0].0.clone();
        let mut best_first_action = beam[0].1.clone();
        for (state, first_action) in beam {
            if state.score() > best_state.score() {
                best_state = state;
                best_first_action = first_action;
            }
        }

        best_first_action
    }
    /* }}} */

}
/* }}} */


struct State {}

// impl StateTrait for State {}


