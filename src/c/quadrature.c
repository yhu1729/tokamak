#include "quadrature.h"

int
tkmk_integrate(
  double* output, enum tkmk_QuadratureType quadrature_type, const int order,
  tkmk_IntegrateF f, const double x_min, const double x_max) {
  int n_max;
  const double* point;
  const double* weight;
  if (quadrature_type == GAUSS_LEGENDRE) {
    if (order <= 1) {
      n_max = 1;
      point = tkmk_quadrature_gauss_legendre_point_1;
      weight = tkmk_quadrature_gauss_legendre_weight_1;
    } else if (order <= 3) {
      n_max = 2;
      point = tkmk_quadrature_gauss_legendre_point_2;
      weight = tkmk_quadrature_gauss_legendre_weight_2;
    } else {
      return 1;
    }
  } else {
    return 1;
  }

  const double dx = x_max - x_min;
  double x;
  *output = 0.0;
  for (int n = 0; n < n_max; ++n) {
    x = (point[n] + 1.0) / 2.0 * dx + x_min;
    *output += weight[n] * f(x);
  }
  *output *= dx / 2.0;

  return 0;
}
