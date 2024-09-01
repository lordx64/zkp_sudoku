use rand::Rng;
use sha2::{Sha256, Digest};

// Simplified 4x4 Sudoku board
type Board = [[u8; 4]; 4];

fn main() {
    let solution: Board = [
        [1, 2, 3, 4],
        [3, 4, 1, 2],
        [2, 1, 4, 3],
        [4, 3, 2, 1],
    ];

    let (commitment, permutation) = prover_commit(&solution);
    let challenge = verifier_challenge();
    let response = prover_respond(&solution, &permutation, challenge);
    let result = verifier_verify(&commitment, challenge, &response);

    println!("Verification result: {}", result);
}

// Prover creates a commitment
fn prover_commit(solution: &Board) -> (String, Board) {
    let mut rng = rand::thread_rng();
    let permutation: Board = [
        [rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4)],
        [rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4)],
        [rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4)],
        [rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4), rng.gen_range(1..=4)],
    ];

    let permuted_solution = apply_permutation(solution, &permutation);
    let commitment = hash_board(&permuted_solution);

    (commitment, permutation)
}

// Verifier generates a challenge
fn verifier_challenge() -> bool {
    rand::random()
}

// Prover responds to the challenge
fn prover_respond(solution: &Board, permutation: &Board, challenge: bool) -> Board {
    if challenge {
        *permutation
    } else {
        apply_permutation(solution, permutation)
    }
}

// Verifier checks the response
fn verifier_verify(commitment: &str, challenge: bool, response: &Board) -> bool {
    if challenge {
        is_valid_permutation(response)
    } else {
        let hashed_response = hash_board(response);
        hashed_response == commitment && is_valid_sudoku(response)
    }
}

// Helper functions
fn apply_permutation(board: &Board, permutation: &Board) -> Board {
    let mut result = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] = permutation[board[i][j] as usize - 1][j];
        }
    }
    result
}

fn hash_board(board: &Board) -> String {
    let mut hasher = Sha256::new();
    for row in board {
        hasher.update(row);
    }
    format!("{:x}", hasher.finalize())
}

fn is_valid_permutation(board: &Board) -> bool {
    for row in board {
        if row.iter().min() != Some(&1) || row.iter().max() != Some(&4) {
            return false;
        }
    }
    true
}

fn is_valid_sudoku(board: &Board) -> bool {
    // Check rows
    for row in board {
        if !is_valid_set(row) {
            return false;
        }
    }

    // Check columns
    for col in 0..4 {
        let column: [u8; 4] = [board[0][col], board[1][col], board[2][col], board[3][col]];
        if !is_valid_set(&column) {
            return false;
        }
    }

    true
}

fn is_valid_set(set: &[u8]) -> bool {
    let mut seen = [false; 5];
    for &num in set {
        if num == 0 || num > 4 || seen[num as usize] {
            return false;
        }
        seen[num as usize] = true;
    }
    true
}