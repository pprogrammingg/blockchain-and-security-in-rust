//! We begin our hands on exploration of state machines with two very simple examples.
//! In these examples, we use actually switch boards as the state machine. The state is,
//! well, just the state of the switches.

use super::StateMachine;

/// This state machine models a single light switch.
/// The internal state, a bool, represents whether the switch is on or not.
pub struct LightSwitch;

/// We model this simple system as a state machine with a single transition - toggling the switch
/// Because there is only a single kind of transition, we can use a unit struct.
impl StateMachine for LightSwitch {
    type State = bool;
    type Transition = ();

    fn next_state(starting_state: &bool, t: &()) -> bool {
        !starting_state
    }
}

/// This second  state machine models two light switches with one weird property.
/// Whenever switch one is turned off, switch two also goes off.
pub struct WeirdSwitchMachine;

/// The state is now two switches instead of one so we use a struct.
#[derive(PartialEq, Eq, Debug)]
pub struct TwoBulbs {
    first_bulb: bool,
    second_bulb: bool,
}

/// Now there are two switches so we need a proper type for the transition.
pub enum Toggle {
    FirstSwitch,
    SecondSwitch,
}

/// We model this system as a state machine with two possible transitions
impl StateMachine for WeirdSwitchMachine {
    type State = TwoBulbs;
    type Transition = Toggle;

    fn next_state(starting_state: &TwoBulbs, t: &Toggle) -> TwoBulbs {
        let mut first_bulb = starting_state.first_bulb;
        let mut second_bulb = starting_state.second_bulb;

        match t {
            // if first switch is toggled and bulb 1 is off, then turn bulb 1  on  bulb 2 is
            // unaffected else if first switch is toggled and bulb 1 is on, then turn
            // both bulb 1 and 2 off else if second switch is toggled, do not impact
            // bulb 1, switch bulb 2
            Toggle::FirstSwitch => {
                first_bulb = !first_bulb;

                if !first_bulb {
                    second_bulb = false;
                }

                TwoBulbs {
                    first_bulb,
                    second_bulb,
                }
            }
            Toggle::SecondSwitch => {
                second_bulb = !starting_state.second_bulb;

                TwoBulbs {
                    first_bulb,
                    second_bulb,
                }
            }
        }
    }
}

#[test]
fn sm_1_light_switch_toggles_off() {
    assert!(!LightSwitch::next_state(&true, &()));
}

#[test]
fn sm_1_light_switch_toggles_on() {
    assert!(LightSwitch::next_state(&false, &()));
}

#[test]
fn sm_1_two_switches_first_goes_on() {
    let state = TwoBulbs {
        first_bulb: false,
        second_bulb: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoBulbs {
            first_bulb: true,
            second_bulb: false,
        }
    );
}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_on() {
    // This is the special case. We have to make sure the second one goes off with it.
    let state = TwoBulbs {
        first_bulb: true,
        second_bulb: true,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoBulbs {
            first_bulb: false,
            second_bulb: false,
        }
    );
}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_off() {
    // This is adjacent to the special case. We have to make sure the second one stays off.
    let state = TwoBulbs {
        first_bulb: true,
        second_bulb: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoBulbs {
            first_bulb: false,
            second_bulb: false,
        }
    );
}

#[test]
fn sm_1_two_switches_second_goes_on() {
    let state = TwoBulbs {
        first_bulb: false,
        second_bulb: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::SecondSwitch),
        TwoBulbs {
            first_bulb: false,
            second_bulb: true,
        }
    );
}

#[test]
fn sm_1_two_switches_second_goes_off() {
    let state = TwoBulbs {
        first_bulb: true,
        second_bulb: true,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::SecondSwitch),
        TwoBulbs {
            first_bulb: true,
            second_bulb: false,
        }
    );
}
