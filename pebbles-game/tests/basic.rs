use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn initialization() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    
    // Test valid initialization
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let res = program.send(2, init_config);
    assert!(!res.main_failed());
    
    // Test invalid initialization - zero pebbles
    let invalid_init = PebblesInit {
        pebbles_count: 0,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let res = program.send(2, invalid_init);
    assert!(res.main_failed());
}

#[test]
fn game_actions() {
    let sys = System::new();
    let program = Program::current(&sys);
    
    // Initialize game
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    program.send(2, init_config);
    
    // Test valid move
    let res = program.send(2, PebblesAction::Turn(2));
    assert!(!res.main_failed());
    
    // Test invalid move - exceeds max pebbles per turn
    let res = program.send(2, PebblesAction::Turn(4));
    assert!(res.main_failed());
}

#[test]
fn test_give_up() {
    let sys = System::new();
    let program = Program::current(&sys);
    
    // Initialize game
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    program.send(2, init_config);
    
    // Test surrender action
    let res = program.send(2, PebblesAction::GiveUp);
    assert!(!res.main_failed());
    
    let state: GameState = program.read_state().expect("Failed to read state");
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_restart() {
    let sys = System::new();
    let program = Program::current(&sys);
    
    // Initialize game
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    program.send(2, init_config);
    
    // Test game restart
    let restart_config = PebblesAction::Restart {
        pebbles_count: 15,
        max_pebbles_per_turn: 2,
        difficulty: DifficultyLevel::Hard,
    };
    let res = program.send(2, restart_config);
    assert!(!res.main_failed());
    
    let state: GameState = program.read_state().expect("Failed to read state");
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 2);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
}

#[test]
fn test_hard_mode_strategy() {
    let sys = System::new();
    let program = Program::current(&sys);
    
    // Initialize game in hard mode
    let init_config = PebblesInit {
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Hard,
    };
    program.send(2, init_config);
    
    // Test AI strategy
    let state: GameState = program.read_state().expect("Failed to read state");
    if state.first_player == Player::Program {
        // If AI goes first, verify its first move
        let remaining = state.pebbles_remaining;
        assert!(remaining < 20); // AI should have made a move
    }
} 
