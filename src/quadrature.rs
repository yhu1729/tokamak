use tokamak::lib_c;

pub enum QuadratureType {
    GaussLegendre,
}

pub fn integrate(
    quadrature_type: QuadratureType,
    order: i32,
    f: lib_c::tkmk_IntegrateF,
    x_min: f64,
    x_max: f64,
) -> Result<f64, String> {
    let mut output: f64 = 0.0;
    let tkmk_quadrature_type = match quadrature_type {
        QuadratureType::GaussLegendre => lib_c::tkmk_QuadratureType_GAUSS_LEGENDRE,
    };

    unsafe {
        let error =
            lib_c::tkmk_integrate(&mut output, tkmk_quadrature_type, order, f, x_min, x_max);
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
        return x * 2.0;
    }

    #[test]
    fn test_integrate() {
        let output = integrate(
            QuadratureType::GaussLegendre,
            0,
            Some(integrate_f1),
            0.0,
            1.0,
        )
        .expect("integrate");

        assert_eq!(output, 1.0);
    }
}
