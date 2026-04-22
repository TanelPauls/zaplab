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
        <Slider label="Width" min=3 max=25 value=width />
        <Slider label="Height" min=3 max=25 value=height />

        <div
            id="maze"
            style=move || {
                let (h, _) = maze.get();
                format!(
                    "grid-template-columns: repeat({}, 40px);",
                    h[0].len()
                )
            }
        >
            {
                move || {
                    let (h, v) = maze.get();

                    let height = h.len() - 1;
                    let width = h[0].len();

                    (0..height)
                        .flat_map(|r| {
                            let h = h.clone();
                            let v = v.clone();
                            (0..width).map(move |c| {
                                let mut classes = vec!["cell"];

                                if h[r][c] { classes.push("top"); }
                                if h[r + 1][c] { classes.push("bottom"); }
                                if v[r][c] { classes.push("left"); }
                                if v[r][c + 1] { classes.push("right"); }

                                view! {
                                    <div class=classes.join(" ")></div>
                                }
                            })
                        })
                        .collect_view()
                }
            }
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

        if row == height - 1 {
            // FINAL ROW: force merge all adjacent cells with different sets, no vertical openings
            for col in 0..width - 1 {
                if sets[col] != sets[col + 1] {
                    v[row][col + 1] = false;
                    let old = sets[col + 1];
                    let new = sets[col];
                    for i in 0..width {
                        if sets[i] == old { sets[i] = new; }
                    }
                }
            }
            break;
        }

        // --- STEP 2: vertical connections ---
        let mut down_open = vec![false; width];

        for i in 0..width {
            if !down_open.iter().enumerate().any(|(j, &b)| b && sets[j] == sets[i]) {
                let candidates: Vec<usize> = (0..width)
                    .filter(|&j| sets[j] == sets[i])
                    .collect();
                let chosen = candidates[rng.random_range(0..candidates.len())];
                down_open[chosen] = true;
            }
        }

        for i in 0..width {
            if rng.random_bool(0.5) { down_open[i] = true; }
        }

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

    (h, v)
}