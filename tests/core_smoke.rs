use sts2core::content::{card, enemy};
use sts2core::{begin_combat, legal_actions, step, Action, State};

fn sample_state(seed: u64) -> State {
    let mut s = State::new(80, seed);
    s.add_enemy(enemy::DUMMY, 100);
    for _ in 0..5 {
        s.add_card(card::STRIKE, 0, 0);
    }
    s
}

#[test]
fn begin_combat_is_deterministic_for_same_state() {
    let s = sample_state(12345);
    assert_eq!(begin_combat(s), begin_combat(s));
}

#[test]
fn step_is_deterministic_for_same_state_and_action() {
    let s = begin_combat(sample_state(7));
    let (actions, n) = legal_actions(&s);
    assert!(n > 0);
    let action = actions[0];
    assert_eq!(step(s, action), step(s, action));
}

#[test]
fn illegal_play_is_a_noop() {
    let s = State::new(80, 1);
    let after = step(s, Action::PlayCard { hand: 0, target: 0 });
    assert_eq!(after, s);
}

#[test]
fn state_stays_small_copy_value() {
    assert!(std::mem::size_of::<State>() <= 4096);
    let s = State::new(80, 42);
    let _copy = s;
    let _second_copy = s;
}
