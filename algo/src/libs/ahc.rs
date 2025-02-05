
type Action = Vec<char>;
type Score = i64;

#[derive(Clone, Debug)]
struct State {
}

impl State {
}

impl StateTrait for State {

    fn forward(&mut self, action: &Action) {
    }

    fn score(&self) -> Score {
        0
    }

    fn legal_actions(&self) -> Vec<Action> {
        Vec::new()
    }

    fn is_done(&self) -> bool {
        false
    }

    fn dbg(&self) {
    }

}

/* {{{ StateTrait */
#[allow(dead_code)]
trait StateTrait {
    fn forward(&mut self, action: &Action);
    fn score(&self) -> Score;
    fn legal_actions(&self) -> Vec<Action>;
    fn is_done(&self) -> bool;
    fn dbg(&self);
}
/* }}} */

/* PartialEq, Eq, PartiaoOrd, Ord for State {{{ */
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score().eq(&other.score())
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}
/* }}} */

/* {{{ select_action */
#[allow(dead_code)]
mod select_action {
    use super::*;
    use rand::Rng;

    pub fn random_action<S: StateTrait>(state: &S) -> Option<Action> {
        let legal_actions: Vec<Action> = state.legal_actions();

        if legal_actions.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen::<usize>() % legal_actions.len();
        Some(legal_actions[random_index].clone())
    }

    pub fn greedy_action<S: StateTrait + Clone>(state: &S) -> Option<Action> {
        let legal_actions = state.legal_actions();
        let (mut best_score, mut best_action) = (state.score(), None);
        for i in 0..legal_actions.len() {
            let mut nstate = state.clone();
            nstate.forward(&legal_actions[i]);
            let nscore = nstate.score();
            if nscore > best_score {
                best_score = nscore;
                best_action = Some(i);
            }
        }
        if let Some(best_action) = best_action {
            return Some(legal_actions[best_action].clone());
        } else {
            return None;
        }
    }

    pub fn beam_search_action<S: StateTrait + Clone + Ord + PartialOrd>(
        state: &S, beam_width: usize, beam_height: usize
    ) -> Option<Action> {
        let first_legal_actions = state.legal_actions();

        let mut beam: Vec<(S, usize)> = vec![];
        let mut nbeam: Vec<(S, usize)> = vec![];

        for i in 0..first_legal_actions.len() {
            let mut nstate = state.clone();
            nstate.forward(&first_legal_actions[i]);
            beam.push((nstate, i));
        }
        beam.sort();
        beam.reverse();
        while beam.len() > beam_width {
            beam.pop().unwrap();
        }

        if beam.is_empty() || beam[0].0.score() == state.score() {
            return None;
        }

        for _ in 0..beam_height {
            for (state, idx) in &beam {
                for action in &state.legal_actions() {
                    let mut nstate = state.clone();
                    nstate.forward(action);
                    nbeam.push((nstate, *idx));
                }
            }
            nbeam.sort();
            nbeam.reverse();
            while nbeam.len() > beam_width {
                nbeam.pop().unwrap();
            }
            std::mem::swap(&mut beam, &mut nbeam);
            nbeam.clear();
        }

        if beam.is_empty() {
            None
        } else {
            Some(first_legal_actions[beam[0].1].clone())
        }
    }
}
/* }}} */
