#pragma once

enum tkmk_QuadratureType {
  GAUSS_LEGENDRE,
};

const double tkmk_quadrature_gauss_legendre_point_1[1] = {
  0.0,
};
const double tkmk_quadrature_gauss_legendre_point_2[2] = {
  -0.57735026918962584208,
  +0.57735026918962584208,
};
const double tkmk_quadrature_gauss_legendre_weight_1[1] = {
  2.0,
};
const double tkmk_quadrature_gauss_legendre_weight_2[2] = {
  1.0,
  1.0,
};

typedef double (*tkmk_IntegrateF)(const double x);

int tkmk_integrate(
  double* output, enum tkmk_QuadratureType quadrature_type, const int order,
  tkmk_IntegrateF f, const double x_min, const double x_max);
