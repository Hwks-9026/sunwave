module core {
    sign := |val| val == 0 ? 0 : val > 0 ? 1 : -1;
    export sign;

    abs := |val| val * sign(val);
    export abs;

    clamp := |val, min, max| val > max ? max : val < min ? min : val;
    export clamp;

    lerp := |min, max, spacing| min + (max - min) * clamp(spacing, 0, 1);
    export lerp;

    modulo := |a, b| {
        b == 0 ? {
            0
        } : 
        {  
            abs_b := abs(b);
            a := loop(a := a) {
                a >= abs_b ? recur(a - abs_b) : a
            };
            a := loop(a := a) {
                a <= (0-abs_b) ? recur(a + abs_b) : a
            };
            a
        };
    };
    export modulo;

    t_avg := |vals_tuple| {
        i_sum := loop(i := 0, sum := 0) {
            #vals_tuple <= i ? (i, sum) : recur(i + 1, sum + vals_tuple.i)
        };
        i_sum.1 / i_sum.0
    };
    export t_avg;

}
export core;

module constants {
    PI := 3.141592653589793;
    export PI;
    
    eps := 0.0000001;
    export eps;
}
export constants;

module trig {
    PI := constants.PI;
    
    cos := |t_radians| {
        t := core.modulo(t_radians, 2 * PI);
        t := t < 0 ? t + 2 * PI : t;
        sign :=  t <= PI / 2 ? {
            1
        } :
        {
            t <= 3 * PI / 2 ? {
                -1
            } :
            {
                1
            }
        };
        t := t <= PI / 2 ? {
            t
        } :
        {
            t <= PI ? {
                PI - t
            } :
            {
                t <= 3 * PI / 2 ? {
                    t - PI
                } :
                {
                    (2 * PI) - t
                }

            }
        };

        sum := loop(term := 1, sum := 1, k := 1) {
            core.abs(term) < constants.eps ? sum : {
                term := term * (t * (0-t))/ (((2 * k) - 1) * 2 * k);
                recur(term, sum + term, k + 1);
            }
        };
        sign * sum
    };
    export cos;

    sin := |t_radians| cos(t_radians - PI/2);
    export sin;

    tan := |t_radians| sin(t_radians) / cos(t_radians);
    export tan;
}
export trig;

module calculus {
    derivative := |f| |x| (f(x + core.eps) - f(x)) / (constants.eps);
    export derivative;

    derivate := | f, x | derivative(f)(x); 
    export derivate;
   
    rectangles_per_x := 500;
    integral := |f, a, b, n| {
        loop(i := 0, h := (b - a) / n, total_area := 0) { 
            i >= n ? total_area : recur(i + 1, h, total_area + f(a + (i + 0.5) * h) * h)
        };
    };
    int := |f, x1, x2| integral(f, x1, x2, ((x2-x1) * rectangles_per_x));
    export int;
}
export calculus;


module roots {
    solve := |f, guess, tolerance| {
        df := calculus.derivative(f);
        loop(x := guess) {
            core.abs(f(x)) < tolerance 
                ? x 
                : recur(x - f(x) / df(x))
        }
    };
    export solve;
}
export roots;
