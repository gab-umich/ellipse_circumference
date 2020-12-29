# ellipse_circumference
Computes ellipse circumference using polar and Cartesian integral approximation. 

## Points of interest
- Polar integral approximation appears to out-perform Cartesian integral approximation on my computer in terms of accuracy: while per iteration time is around 4 times slower, the convergence rate is much better.
- All approximation trials used lambdas (i.e. closures) to take advntage of the read only reference property of closure reference.
