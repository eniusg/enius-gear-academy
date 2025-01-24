use gtest::{Program, System};
use pebbles_game_io::*;

const USER_ID: u64 = 2;

#[test]
fn initialization() {
    let sys = System::new();
    sys.init_logger();

    // 为用户添加足够的 gas 费用
    sys.mint_to(USER_ID, 99999999999999999);
    let program = Program::current(&sys);

    // Test valid initialization
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let _res = program.send(USER_ID, init_config);
    sys.run_next_block();

    // Test invalid initialization - zero pebbles
    let invalid_init = PebblesInit {
        pebbles_count: 0,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let _res = program.send(USER_ID, invalid_init);
    sys.run_next_block();
}

#[test]
fn game_actions() {
    let sys = System::new();
    sys.init_logger();

    let program = Program::current(&sys);
    sys.mint_to(USER_ID, 99999999999999999);

    // Initialize game
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    program.send(USER_ID, init_config);
    sys.run_next_block();

    // Test valid move
    let _res = program.send(USER_ID, PebblesAction::Turn(2));
    sys.run_next_block();

    // Test invalid move - exceeds max pebbles per turn
    let _res = program.send(USER_ID, PebblesAction::Turn(4));
    sys.run_next_block();
}

#[test]
fn test_give_up() {
    let sys = System::new();
    sys.init_logger();

    let program = Program::current(&sys);
    sys.mint_to(USER_ID, 99999999999999999);

    // Initialize game
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    program.send(USER_ID, init_config);
    sys.run_next_block();

    // Test surrender action
    let _res = program.send(USER_ID, PebblesAction::GiveUp);
    sys.run_next_block();

    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_restart() {
    let sys = System::new();
    sys.init_logger();

    let program = Program::current(&sys);
    sys.mint_to(USER_ID, 99999999999999999);

    // Initialize game
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    program.send(USER_ID, init_config);
    sys.run_next_block();

    // Test game _restart
    let _restart_config = PebblesAction::Restart {
        pebbles_count: 15,
        max_pebbles_per_turn: 2,
        difficulty: DifficultyLevel::Hard,
    };
    let _res = program.send(USER_ID, _restart_config);
    sys.run_next_block();

    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 2);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
}

#[test]
fn test_hard_mode_strategy() {
    let sys = System::new();
    sys.init_logger();

    let program = Program::current(&sys);
    sys.mint_to(USER_ID, 99999999999999999);

    // Initialize game in hard mode
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Hard,
    };
    program.send(USER_ID, init_config);
    sys.run_next_block();

    // Test AI strategy
    let state: GameState = program.read_state(()).expect("Failed to read state");
    if state.first_player == Player::Program {
        // If AI goes first, verify its first move
        let remaining = state.pebbles_remaining;
        assert!(remaining < 20); // AI should have made a move
    }
}
