#[derive(Copy, Clone, Debug, PartialEq, approx_some_other::AbsDiffEq)]
struct Point(f32);

fn ode(x: &Point, dx: &mut Point, spring_constant: f32) {
    dx.0 = -spring_constant * x.0;
}

fn main() {
    let mut p1 = Point(1.0);
    let mut dp = Point(0.0);
    let spring_constant = 0.1;
    let dt = 0.03;

    let mut results = vec![p1];
    let mut increments = vec![];
    for iteration in 0..1000 {
        ode(&p1, &mut dp, spring_constant);
        increments.push(dp);
        dp.0 = 0.0;

        let n = increments.len();
        if n == 1 {
            let dp0 = increments[0].0;
            p1.0 += dt * dp0;
        } else if n == 2 {
            let dp0 = increments[0].0;
            let dp1 = increments[1].0;
            p1.0 += dt * (3. / 2. * dp0 - 1. / 2. * dp1)
        } else if n >= 3 {
            let dp0 = increments[n - 1].0;
            let dp1 = increments[n - 2].0;
            let dp2 = increments[n - 3].0;
            p1.0 += dt * (23. / 12. * dp2 - 16. / 12. * dp1 + 5. / 12. * dp0);
        }
        if iteration % 10 == 0 {
            results.push(p1);
        }
    }

    let width = results.len();
    let height = 30;
    let mut disp = vec![vec![" ".to_string(); width + 2]; height];

    for (i, res) in results.iter().enumerate() {
        let cutoff = (height as f32 * res.0).round() as usize;
        for (k, d) in disp.iter_mut().rev().enumerate() {
            if cutoff >= k {
                d[i] = "#".to_string();
            }
        }
    }
    for (k, d) in disp.iter_mut().enumerate() {
        d[width + 1] = format!("{:7.3}", 1. - k as f32 / height as f32);
    }

    /* for k in 0..height {
        for (i, r) in results.iter().enumerate() {
            let v = (height as f32 * r.0).round();
            if k as f32 <= v {
                disp[i][k] = "#";
            }
        }
    }*/

    for row in disp.iter() {
        println!("{}", row.join(""));
    }
}
