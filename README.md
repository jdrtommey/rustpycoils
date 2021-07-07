# rustpycoils
[![Actions Status](https://github.com/jdrtommey/rustpycoils/workflows/Test/badge.svg)](https://github.com/jdrtommey/rustpycoils/actions)
[![Actions Status](https://github.com/jdrtommey/rustpycoils/workflows/LintFormat/badge.svg)](https://github.com/jdrtommey/rustpycoils/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Python wrapper around [rustycoils](https://www.github.com/jdrtommey/rustycoils).

This module provides an implementation of an algorithm for finding the off-axis magnetic field of solenoidal systems of current loops.

An analytic solution to the ideal wire loop is readily available and can be computed using elliptical integrals which are also readily available in libraries such as scipy. To approximate other solenoidal magnetic field systems, such as solenoids and Helmholtz coils, this basic primitive can be duplicated however this will start to slow down as more primitives are added. Additionally provided the magnetic field is only required to be accuratly known close to the central axis of symmetry a faster power series method can be employed to determine the field for any system for which the magnetic field on the axis is known. 

This module impliments the primitive objects discussed in [Off-axis expansion solution of Laplace's equation: Application to accurate and rapid calculation of coil magnetic fields](https://ieeexplore.ieee.org/document/760416) which crucially have analytic derivatives to arbitrary powers allowing for fast and accurate higher order derivatives of the total magnetic field to be computed. Using these primitives to build up the total system allows for magnetic systems of coils to be accuratly and rapidly be computed close to the axis. 

This is not a general competitor for modules such as [magpylib](https://magpylib.readthedocs.io/en/latest/) which offer analytic expressions for magnetic fields based on primitives, and provided there is no material response, give exact solutions. It is instead useful when a magnetic field with cylindrical symmetry is being calculated for use in an atomic beam simulation (my use case). In particular when only the region close to the center of the coil is needed. 
