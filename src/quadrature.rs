use crate::assert_eq_within;
use crate::constant;
use tokamak::lib;

pub enum QuadratureType {
    GaussLegendre,
}

pub fn integrate(
    quadrature_type: QuadratureType,
    order: i32,
    f: lib::tkmk_IntegrateF,
    x_min: f64,
    x_max: f64,
) -> Result<f64, String> {
    let mut output: f64 = 0.0;
    let tkmk_quadrature_type = match quadrature_type {
        QuadratureType::GaussLegendre => lib::tkmk_QuadratureType_GAUSS_LEGENDRE,
    };

    unsafe {
        let error = lib::tkmk_integrate(&mut output, tkmk_quadrature_type, order, f, x_min, x_max);
        if error != 0 {
            return Err("tkmk_integrate".to_string());
        }
    }

    return Ok(output);
}

#[cfg(test)]
mod test {
    use super::*;

    unsafe extern "C" fn integrate_f1(x: f64) -> f64 {
        return x * 3.0;
    }

    unsafe extern "C" fn integrate_f2(x: f64) -> f64 {
        return 1.0 + x * 3.0 + x * x * 4.0;
    }

    #[test]
    fn test_integrate() {
        let output_f1 = integrate(
            QuadratureType::GaussLegendre,
            1,
            Some(integrate_f1),
            0.0,
            3.0,
        )
        .expect("integrate");
        assert_eq!(output_f1, 13.5);
        let output_f2 = integrate(
            QuadratureType::GaussLegendre,
            2,
            Some(integrate_f2),
            -1.0,
            2.0,
        )
        .expect("integrate");
        assert_eq_within!(output_f2, 19.5, constant::EPSILON_F64);
    }
}
