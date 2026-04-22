mod slider;

use leptos::prelude::*;
use slider::Slider;
use rand::prelude::*;

#[component]
fn App() -> impl IntoView {
    let width = RwSignal::new(5u32);
    let height = RwSignal::new(5u32);

    let maze = create_memo(move |_| {
        ellers(width.get() as usize, height.get() as usize)
    });

/*     leptos::logging::log!("Horizontal walls:");
    leptos::logging::log!("{:?}", h);
    leptos::logging::log!("Vertical walls:");
    leptos::logging::log!("{:?}", v); */

    view! {
        <h1>"WELCOME"</h1>
        <Slider label="Width" min=3 max=10 value=width />
        <Slider label="Height" min=3 max=10 value=height />
        <div>
            {move || {
                let (h, v) = maze.get();
                format!("{:?}", h)
            }}
        </div>
    }
}



#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);


}

fn ellers(width: usize, height: usize) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let mut rng = rand::rng();

    // walls: true = wall exists
    let mut h = vec![vec![true; width]; height + 1];
    let mut v = vec![vec![true; width + 1]; height];

    // current row set ids
    let mut sets: Vec<usize> = (0..width).collect();
    let mut next_set = width;

    for row in 0..height {
        // --- STEP 1: horizontal joins ---
        for col in 0..width - 1 {
            if sets[col] != sets[col + 1] && rng.random_bool(0.5) {
                v[row][col + 1] = false;

                let old = sets[col + 1];
                let new = sets[col];

                // merge sets (O(n), fine for grids)
                for i in 0..width {
                    if sets[i] == old {
                        sets[i] = new;
                    }
                }
            }
        }

        // --- STEP 2: vertical connections ---
        let mut down_open = vec![false; width];

        // ensure at least one per set
        for i in 0..width {
            if !down_open.iter().enumerate().any(|(j, &b)| b && sets[j] == sets[i]) {
                let candidates: Vec<usize> = (0..width)
                    .filter(|&j| sets[j] == sets[i])
                    .collect();

                let chosen = candidates[rng.random_range(0..candidates.len())];
                down_open[chosen] = true;
            }
        }

        // random additional openings
        for i in 0..width {
            if rng.random_bool(0.5) {
                down_open[i] = true;
            }
        }

        // apply vertical openings + build next row sets
        let mut new_sets = vec![0; width];

        for i in 0..width {
            if down_open[i] {
                h[row + 1][i] = false;
                new_sets[i] = sets[i];
            } else {
                new_sets[i] = next_set;
                next_set += 1;
            }
        }

        sets = new_sets;
    }

    // --- FINAL ROW: force full connection ---
    let last = height - 1;
    for col in 0..width - 1 {
        if sets[col] != sets[col + 1] {
            v[last][col + 1] = false;

            let old = sets[col + 1];
            let new = sets[col];

            for i in 0..width {
                if sets[i] == old {
                    sets[i] = new;
                }
            }
        }
    }

    (h, v)
}