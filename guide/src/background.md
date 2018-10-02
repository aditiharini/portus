# Background

The focus here is to explain the CCP programming model, so we'll be implementing a very simple scheme: AIMD (additive-increase multiplicative-decrease). Before we even start talking about CCP, let's briefly go over exactly how the algorithm works and what kind of behavior we expect to see. We assume a basic familiarity with the problem of congestion control. If you need some background, [Van Jacobson's paper](http://web.mit.edu/6.829/www/currentsemester/papers/vanjacobson-congavoid.pdf) is a good place to start.
