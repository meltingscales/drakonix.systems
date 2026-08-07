---
title: "On automating the OSCP"
date: 2026-08-07T02:29:10Z
draft: false
tags: ["rant","cybersecurity","oscp"]
---

I've recently been studying for the OSCP, aka the PEN-200.

It's a seemingly daunting test where you get 24 hours to hack into 3 Linux, 3 Windows Active Directory boxes.

I've cobbled together about 30 boxes I've hacked into from OFFSEC Proving Grounds,

as well as a handful from HackTheBox (HTB).

As I've been going through the exercises, it makes me feel that large parts of the manual work could be 
fully automated.

Of course, that's banned on the test - the real expertise comes from knowing how to enumerate a target
and do the hacking yourself.

But especially the windows AD traversal - getting an AD graph with BloodHound-python and loading it into
bloodhound's web interface...

It feels like a lot of it could be automated. I'm sure there are companies (esp. with the advent of AI coding)
that are doing that right now.

Anyways, it's an interesting problem to solve. I fear (slash hope) within a few years it will be a fully solved
problem.

If you're curious about my attempts, see /boxes/ and `report.md` files in this repo :)

<https://github.com/meltingscales/oscp/tree/main/boxes>
