# rustpycoils
[![Actions Status](https://github.com/jdrtommey/rustpycoils/workflows/LintFormat/badge.svg)](https://github.com/jdrtommey/rustpycoils/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Python wrapper around [rustycoils](https://www.github.com/jdrtommey/rustycoils).

This module provides an implementation of an algorithm for finding the off-axis magnetic field of solenoidal systems of current loops.

An analytic solution to the ideal wire loop is readily available and can be computed using elliptical integrals which are also readily available in libraries such as scipy. To approximate other solenoidal magnetic field systems, such as solenoids and Helmholtz coils, this basic primitive can be duplicated however this will start to slow down as more primitives are added. Additionally provided the magnetic field is only required to be accurately known close to the central axis of symmetry a faster power series method can be employed to determine the field for any system for which the on-axis magnetic field is known. 

This module impliments the primitive objects discussed in [Off-axis expansion solution of Laplace's equation: Application to accurate and rapid calculation of coil magnetic fields](https://ieeexplore.ieee.org/document/760416) which crucially have analytic derivatives to arbitrary powers allowing for fast and accurate higher order derivatives of the total magnetic field to be computed. Using these primitives to build up the total system allows for magnetic systems of coils to be accurately and rapidly be computed close to the axis. 
# Install

Can install using PyPi

```
pip install rustpycoils
```

If pipy does not contain the binaries for your system wheels can be built using [maturin](https://github.com/PyO3/maturin), which requires a Rust compiler. 
```
git clone https://github.com/jdrtommey/rustpycoils/
cd rustpycoils
maturin build --release
```



# Range of Applicitability

This is not a general alternative for modules such as [magpylib](https://magpylib.readthedocs.io/en/latest/) which offer analytic expressions for magnetic fields based on primitives, and provided there is no material response, give exact solutions. It is instead useful when a magnetic field with cylindrical symmetry is being calculated, which needs to be computed fast for a region close to the central axis. This can be highlighted by comparing the speed of the algorithm against the analytical provided in [magpylib](https://magpylib.readthedocs.io/en/latest/) for the ideal wire loop. The test uses a wire loop with a radius of 1m and a current of 1A. 


###### Speed comparision

![Alt text](./benchmark/speed_comparison.png)

As can be seen inset in the left-hand figure this library can compute the magnetic field of a single position around 200 times faster (performed on 4-core 1.4Ghz Intel i5 MacBook pro). The speed then continues to scale better, particularily after around 4000 positions. The speed increase is more dramatic when more  wire loops are included in the calculation, such as to model a solenoid or a pair of helmholtz coils. This is shown on the right where the magnetic field is computed at a single location at the center of the coils as the number of coils increases. 

###### Accuracy Comparison

However, caution needs to be taken to ensure this algorithm is used correctly. Unlike the solution provided by elliptical integrals, this algorthim is not accurate everywhere. The paper from which this library is derived claims accuracies of around 0.1% can be achieved at radial positions 70% of the radius.

![Alt text](./benchmark/accuracy.png)

The agreement between the two libraries for both the axial (left) and radial (right) magnetic fields for positions which are located 70cm away from the middle of the wire loop. The agreement between the two codes saturates at around 1e-8% due to the discreprency in the definition of the vacuum permeability, where magpylib appears to be using 4?? e-7 H/m, while this module uses 1.25663706212 e-6 H/m. 

The agreement for a wide range of axial and radial positions is shown below:

![Alt text](./benchmark/accuracy2.png)

Up to around 20% of the radius of the coil the agreement is at the level of agreement in the vacuum permiability. For radial positions approaching, and larger than, the radius the algorithm performs very poorly unless located very far away along the axial direction. These comparisons only show the accuracies for the given input parameters however they give a good idea of the range of applicability of this aproach. 

# Usage 

The module exposes a single class 
```python

import rustycoils

#object containing primitives sharing a symmetry axis
mycoil = AxialSystem()
```
which defines a symmetry axis. Currently this symmetry axis can only be defined along the three cartesian axes (defaults to x). Eventually this will be arbitrary. 

```python

mycoil.transform_x()
mycoil.transform_y()
mycoil.transform_z()

```

Individual primitive coils can be added to the AxialSystem with a unique string identifier. The units are SI with radius,thickness,length and position in metres and current in Amperes. Positions of primitives relative to the AxialSystem are given from one end for the primitives with length, i.e., a 5m long solenoid at position 2m extends from 2m to 7m.

```python
#define physical parameters
radius = 1.0
thickness = 0.1
current = 1.0
length = 5.0
position = 2.0 //position along the symmetry axis
mycoil.add_loop("loop1",radius,position,current)
mycoil.add_annular("foo",radius,thickness,position,current)
mycoil.add_solenoid("bar",radius,length,position,length)
mycoil.add_coil("coil1",radius,length,thickness,position,current)
```
The parameters controlling these primitives can be be modified by using the functions 
```python
#change radius of the current loop
mycoil.modify_radius("loop1",6.0) "loop1"
#change length of the solenoid "bar"
mycoil.modify_length("bar",3.0)
#change length of the coil "coil1"
mycoil.modify_position("coil1",3.0)
#change thickness of the annular "foo"
mycoil.modify_thickness("foo",1.0)
#change current of the annular "foo"
mycoil.modify_current("foo",1.0)
```

These functions accept keywords to modify multiple primitives at once. Note these keywords can not be used as identifiers for primitives.

| Reserved word  | Meaning |
| -------------  | ------------- |
| *   | Apply to all  |
| LOOP   | Apply to current loops  |
| ANNULAR   | Apply to annulars  |
| SOLENOID   | Apply to solenoids  |
| COIL   | Apply to coils  |


```python
#changes all the current of all primitives
mycoil.modify_current("*",6.0)
#changes all the current of all current loop primitives
mycoil.modify_current("LOOP",6.0)
#changes all the radius of all annnlar primitives
mycoil.modify_radius("ANNULAR",6.0)
#changes all the length of all solenoid primitives
mycoil.modify_length("SOLENOID",6.0)
#changes all the length of all coil primitives
mycoil.modify_thickness("COIL",6.0)
```

The magnetic field in each of the cartesian directions can be computed using an input of a numpy array. Currently requires a 2D numpy array 
to work which is a little tedious for a single field value.
```python
fields = mycoil.get_field(np.asarray([x,y,z]).reshape(3,-1),1e-18)
```
where 1e-18 is the tolerance to stop including additional terms in the power expansion.
