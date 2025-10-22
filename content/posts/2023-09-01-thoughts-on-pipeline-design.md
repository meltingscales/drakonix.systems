---
title: Brainstorming on CICD Pipeline Design
date: '2023-09-01'
tags: [programming, cicd, vulnerabilities, cybersecurity, automation]
---

# Brainstorming on CICD Pipeline Design

What are the questions that I should be asking myself when I'm thinking about how to automate security in a CICD
pipeline?

What about a CICD pipeline without thinking about security?

- What should CICD Pipelines do?
- What shouldn't CICD Pipelines do?

These feel too general. These might be valuable to explore, but I think that making them more specific will be better.

The questions that I ask myself are important because they will determine the architecture of the pipeline. I feel like
I'm starting to understand what 2 of my coworkers, Gil and Vishal, are talking about. If I start trying to design a CICD
pipeline with an existing piece of automation, it's not a good starting point. But if I start with the right principles
in mind, building good ideas becomes easier.

## What's the goal of a CICD Pipeline without thinking about security?

Let's brainstorm:

- Code Builds
    - Deterministic
    - Repeatable
    - Well-documented
    - Automated

- Build Environments
    - Deterministic
    - Repeatable
    - Well-documented
    - Automated

- Deployments
    - Deterministic
    - Repeatable
    - Well-documented
    - Automated

I have a weird feeling. I want to stop this thought experiment. I think that baking security into this is necessary. The
reason...Baking in security raises the bar for a lot of these things. 

"Security" isn't even a good word for it. Hell, malware isn't even a good term, and most people misunderstand it. It's
just gaps in processes, lapses in thought. Software and data are just logical structures, and "security bugs" are
literally the same as "this organization of bits in memory is not expected". Instead of the word "security", now my thoughts are morphing into "Automation, repeatability, open-source design, normalization, sharing knowledge, etc."

I have a gut feeling. The more things that we can automate, normalize, track, and govern in an automated and extensible
manner, the better that security will get. 

## What the hell even is "security"?

Or, what could contribute positively to the "security" of an organization?

- Automating processes that could be manual
  - Test, release, etc;
- Repeatability and consistency
- Avoid security by obscurity
- Avoid siloes
- Promote best practices among software developers
    - Give them high-quality templates
    - Encourage developers to self-organize and share their software solutions
- Encourage collaboration across different business lines

### "Security" in CICD, in the flesh?

What could these look like in practice?

- Traceability for all of these
- Repeatability for all of these
- The ability to link a "Modification" (Deployment, config change, Git SHA) to the resource itself
  - For example, if we make a commit to our VCS that breaks a load balancer in DEV, 
- Automated code builds
- Automated code coverage
- Automated unit tests
- Automated security tests
  - Pentests
  - SAST
  - DAST
  - IAST
  - SCA (OSS)
- Automated deployments
  - Shell scripts? `kubectl`? Hmmm...
- Declarative configuration
    - ex: NixOS, Terraform, Kubernetes, Helm Charts
    - ex2: Declarative Infrastructure, or Infra as Code

These aren't good just for CICD... 

Having things like this will have long-lasting positive effects across multiple different facets of an organization.

These make deployments faster, they make them more repeatable, easier to test, bugs are uncovered faster, etc.

The downside of all of this stuff is getting it all set up (the right way), training people to use it, upgrading it, instrumenting it, debugging it, etc.

Hell...you don't even have to do it all at once. Pick one and start! The benefits for application developers are fantastic. Bugs become traceable, deployments are faster, configuration changes get recorded, security issues are seen and fixed faster, etc.

# Inspiration

https://gilyehuda.mystrikingly.com/

https://patterns.innersourcecommons.org/

https://www.linkedin.com/pulse/culture-behaviors-innersource-three-part-blog-series-3-gil-yehuda/

https://www.linkedin.com/pulse/what-does-open-source-mean-gil-yehuda/