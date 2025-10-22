---
layout: post
title: "Certified Secure Software Lifecycle Professional (CSSLP) Notes"
date: 2020-06-15
categories: [programming, certification, security]
toc: true
---

## Preamble

<https://www.isc2.org/Certifications/CSSLP>

This is a collection of notes I've taken for the CSSLP. Hopefully they
are useful to you as well!

I made a program to study for this test. Feel free to use it.

<https://github.com/HenryFBP/pyconsolequiz>

## Secure Software Concepts

### Core Concepts

- Main concept is about restricting user access

#### Confidentiality

- Keep info away from people who don't NEED to know it
- Secret info remains secret

- Must understand what data needs to be kept secret
- In order to do this, data must be classified, e.g.
  - Public
  - Available to everyone
  - Nonpublic
    - Restricted in some way
    - Who can access this data?

```
publicly disclosed? --> no --> disclosed by roles? --> no --> [restricted]
      |                                 |
     yes                               yes
      |                                 |
  [public]                        [confidential]
```

##### Data protection

- Confidentiality Controls
  - _Masking_: Deleting parts of data, i.e. removing first 12 digits of
    credit card
  - Secret writing
    - Covert (hidden among other data)  
      The placement of the data is itself hidden. May also include
      encryption/ciphers.
      - Steganography
      - Digital Watermarking
    - Overt (visible) The data we're protecting is in plain sight, but
      the method of decrypting/deciphering is not known to others.
      - Encryption
      - Hashing (i.e. passwords)

##### Where do we need confidentiality?

- In transit
  - Unprotected Networks
- In processing
  - Data in memory
- In storage
  - Data at rest

#### Integrity

Integrity means the data is protected from any unauthorized change.

Changes must be done by an authorized user. This means we need an
authorization scheme, and a way to determine that data is 'authentic',
to have proof that it was made by

```
minimal/no damage upon change? --> no --> significant damage? --> no --> critical damage
                |                                  |                            |
               yes                                yes                         [high]
                |                                  |
              [low]                            [medium]
```

- Ensure functionality
- Ensure accuracy
- Completeness and consistency (i.e. incomplete update)

##### Input validation

- Users inputting data accidentally
- Many types of accidental or intentional errors
- 'Injection flaw', i.e. SQLi
  - Compromise input of data OR entire system
- Input validation ensures data integrity

##### Parity Bit Checking

- Detects errors or changes made during transit
- An extra bit is added to a piece of data
  - It is `1` if the sum of `1`s in the data is odd, `0` if it's even.
- Fast to calculate, but...
- You can still change data to manipulate the parity check to be
  successful
  - ...all powers of 2 are even (except 2^0), flipping them doesn't
    change the parity.
    - Parity bits are essentially a copy of the last bit AFAICT.

```
0   1   0   0   0   1   1   0  [1] <--- 3 is odd, this parity
                                        bit is 1.
```

##### Cyclic Redundancy Checking (CRC)

- Uses parity bit checking for data integrity
- Good for integrity checking during transmission
- But, CRC can be recalculated and modified

- CRC calc uses 1 bit for polynomial
- Most polynomials are 16/32 bits
- This polynomial is used to calculate a checksum value that is added to
  the data
- More bits = more accuracy = harder to change the data in a way that
  cannot be detected by examining the checksum
- Altering the data results in a mismatch when recalculating the CRC

- CRC are based on cyclic ECC
- CRC =/= hash fn, but very similar in behavior

<https://eklitzke.org/crcs-vs-hash-functions>

##### Hashing

- A hash is a (generally) smaller value derived from performing a
  calculation on a large piece of data
- When putting the large data through a hash fn, the same hash is
  (almost always) going to be returned by the hash function
- `hash(coolio) -> 681`
- When my friend on their computer hashes `coolio`, they also get `681`
- Calculation 'cannot be reversed'
  - ...unless you try every possible input value (brute-force)
  - The ALGORITHM is a ONE-WAY algorithm...hashes CAN BE REVERSED, it's
    just that with strong hashes, this is unfeasible. Try using MD5
    password hashes if a nation-state wants to target you -- They might
    have 10000 GPUs.
- Can ensure confidentiality -- Transmitting a password hash only
  provides the other party _the means to validate a password_, not the
  password itself.
  - This is how NTLM (NT Lan Manager) auth works. Very insecure because
    of OTHER reasons though.
- Many different algos to make hashes
- 'digital fingerprinting'
  - A hash guarantees that the data is intact
  - A digital signature guarantees that the HASH AND DATA were provided
    by a specific entity
  - We know who made it, and that it is intact
  - Example: <http://releases.ubuntu.com/focal/MD5SUMS>
    <http://releases.ubuntu.com/focal/MD5SUMS.gpg>

#### Availability

- Making sure data is available to users
- How valuable is the data?

```
minimal/no impact upon destruction? --> no --> significant impact? --> no --> critical impact
                |                                   |                               |
               yes                                 yes                          [critical]
                |                                   |
            [support]                          [essential]
```

- Low-value data is less critical and is 'more ok' to destroy
- Must ensure no disruption to operation because any interruption could
  make a piece of data unavailable
  - i.e. 'Products' database goes down, and 5,000 stores across America
    depend on that...millions of dollars per minute potentially lost
- Both data and RELATED systems must be protected
  - A breach in any RELATED system could compromise the data

##### Maximum Tolerable Downtime

- Must establish a 'Maximum Tolerable Downtime'
  - MOST systems CAN be down for some amount of time
    - Maybe not nuclear reactors, but bank sites CAN be offline for a
      few hours per day/week.
  - 2 days per week, or '99.9% uptime'
  - Many systems (esp 3rd party) have SLAs (service level agreement)
    that ensures a minimum.

##### Recovery Time Objective

- Must consider RTO when UNPLANNED downtime occurs
- RTO is the maximum amount of time that it takes to recover a system
  from a failure
- This is important when planning SLA lengths
- If we said RTO=1h, and a disaster happened, we should be back up and
  running within 1 hour.

* Targeted duration for recovery
* Explicitly state RTO in SLAs
* Consider this during recovery planning

* Determine the impact of unavailability
  - Can't take orders?
* Measure impact quantitatively and qualitatively
* The way an org uses data will change over time, so both current and
  new data must be considered.

#### Authentication and Authorization 1

- Many ways to authenticate
  - Anonymous
    - Not secure
    - No creds
    - Avoid using this method if you care about securing something
    - Unlinkability
      - You cannot tell WHO performed an action
  - 'Basic'
    - Base64 encoded creds sent over HTTP in every request
    - Basically plaintext creds (encoding is NOT encryption!!!)
      ...so...encrypt your traffic...or don't use this method.
    - Very widely used unfortunately, and common
  - Digest
    - Challenge/response
    - Only password hashes are transmitted
  - Integrated auth
    - Uses challenge-response
    - NTLM auth is integrated with Windows
    - Standalone vs Kerberos v5 auth
  - Client certs
    - Digital Certs
    - Internet/e-commerce
  - Forms
    - Web apps
      - Uname+pw gives the client a auth token to reuse (session token)
    - SSL should be used because uname+pw are transmitted over HTTP.
  - Token-based auth
    - Used with uname+pw
    - Once verified, token is issued
  - Smart Cards
    - Ownership-based auth
    - Creds stored on a microchip
    - Difficult to compromise
      - Needs the password and the smartcard as well
  - Biometrics
    - Unique physical characteristic of user (fingerprints, retina)
    - Can be expensive
    - Suffers from errors (rare though)
      - Type I (False Negative)
      - Type II (False Positive)
    - Detection is complex and errors happen

Forms and basic are different because forms are made by web devs and
basic auth is handled by the webserver software (sent in HTTP headers).

#### Authentication and Authorization 2

Authorization is the act of verifying an entity's permission to perform
an action on an object.

```

subject --- security server ---- permission granted? -- yes --> object access
                                          |
                                         no
                                         |
                                         X
```

- Discretionary Access Control (DAC)
  - Restricts access to object based on identity
  - The task of controlling permissions can be granted to anyone
  - DACs must Maintain an Access Control List (ACL) for the object that
    is getting accessed
    - When someone attempts to access the object, the ACL is checked to
      see if they or one of their groups has permissions to access the
      object.
    - For this to work, the subject (individual) needs to be
      authenticated by a secure server, and their role membership needs
      to be evaluated
      - That then needs to be compared to the ACL to see if they have
        access

* Nondiscretionary Access Control (NDAC)

  - Also controls authorization
  - NDAC is different from DAC because of who can manage the permissions
    - Only the admin or a small mgmt body can control permissions to an
      object
      - This control is systemwide and imposed on many subjects and
        objects
    - Can be installed on many OSes or configured in existing DAC
  - Offers a high degree of protection, but it restricts autonomy and
    involves a lot more administration

* Mandatory Access Control (MAC)

  - Is a form of NDAC
  - Restricts access based on information sensitivity
  - Privileges and formal authorization are still required to access
    objects
  - A single admin body is required to control access as MAC is born
    from NDAC
    - This body provides priviledge and authorization
  - Access is 'multilevel' as information sensitivity is different per
    classification
    - Top Secret data can be viewed by one group, Classified can be
      viewed by another.
  - Information must be PROPERLY CLASSIFIED in order for MAC to be
    useful as an Access Control scheme
  - A common implementation of this is to use Rules to assign the right
    data to the right classifications

* Role-based Access Controls (RBAC)
  - Focus on the job role/function that a person is in to be able to
    assign permissions to objects
  - The role a person is placed in will determine how much trust you are
    giving them
    - i.e. `User 5 -> Store Manager Role` will grant User 5 all the
      permissions that the `Store Manager Role` has
  - Users -or- services can be given Roles
  - Underlying access is granted based on Roles
    - RBAC works with the other AC models and simplifies management
  - This model (RBAC) can work with DAC, NDAC, and MAC

#### Accounting (Auditing)

- Measure activity that happens on a system
  - Who changed what, who accessed what
  - Keep historical access records
  - Records can be used to detect anomalies
  - Records can assist us if we have problems

##### Logging

Audit logs must be stored, and enough resources must be allocated to
create, store, and review logs.

- Resources
  - Create
  - Store
  - View

- Log files alone do not create security
- All critical transactions should be logged

###### Logging requirements

- Who is performing the action
- What action is being performed
- Where is this action being performed
- When was the action performed

#### Non-repudiation

Non-repudiation is being able to prove that a person IS THE ENTITY that
performed an action.

i.e. It is impossible for them to 'repudiate' (deny) that they 'took a  
cookie from jar 28 at 9am on Monday'

If a change happens in an information system, we need to be able to  
apply corrective action to the right person!

- Audit logs must capture enough data (who, where, when, what)

##### Identification

The identification mechanism (aka auth mech) needs to be accurate so  
that someone can't impersonate another user and circumvent  
non-repudiation.

Uname+pw CAN BE IMPERSONATED if someone gets the password of another  
user... All we 'know' at that point is that SOMEONE who knows
`steveba:p@$$w0rd!` logged in at 1am on Friday, not necessarily Steve
Ballmer.

Adding something like a retina scanner requires you to physically be in
possession of the eyeball to authenticate, so you'd need Steve's eye to
log in, which may be harder than getting his credentials.

The easier it is for someone to bypass an authentication mechanism, the
easier it is for the OWNER of a potentially compromised account to
REPUDIATE (deny) any action performed using their account.

"Hey, it wasn't me! My login got stolen"!

versus

"Hey, it wasn't me! My eyeball got stolen"!

- After logon, audit logs must record what actions are performed by who
- Identification of the user will only be as good as the auth system we
  are using

##### Requirements

- Accounting requires a lot of extra space and resources
- Consider security requirements carefully instead of logging to the
  finest level by default
  - Security Requirements
    - Subjects
    - Objects
    - Events
- Complete non-repudiation needs:
  - Logging all actions, subjects, etc
  - LARGE amount of data
  - Likely unnecessary
    - Should focus on critical data

There may be OTHER EVENTS that you need to log in order to protect
critical data -- Things that are not directly related to critical data.

An example of this is somebody who creates a new user and adds them to a
new security group, which could allow a nonpriviledged user to gain
access to critical data by using a different account.

All changes to security groups and users should be logged, as well as
access to data.

### Security Design Principles

These are key components to maximize software security against  
disruption and attacks.

#### Least Privilege

- Fundamental approach
- Minimal access rights
  - Minimum amount of time
- Useful for administering a system
- Limits harm if something is compromised

This is a fundamental approach to security.

Essentially, grant the MINIMUM amount of privileges to accomplish a task.
No more is given.

For a person, this means they get the absolute minimum perms and time to do a task.

Example: An admin's only job is to take and maintain backups. They should be able to back up the 
system, and nothing more.

Least Privilege is a good technique because it minimizes the potential for harm if a person,
credential, system, or anything else were to be compromised.

Often times, data loss is actually due to user error, not malicious intent. LP minimizes this.

##### Need to Know

- Military sec rule
- Limits disclosure of info
- Increased confidentialiy
- Mitigates Risk

Least Priviledge also means that disclosure of data is only given to people
who NEED ACCESS to the data.

This is a basic military security rule and it helps to limit the spread of critical info.

- Who NEEDS to work with this data?
  - versus 'who is ENTITLED to this data', which spreads more info than necessary

This increases confidentiality of the data, which mitigates risk, as LESS PEOPLE 
have access to the data.

##### Modular programming

- Software design technique
- Software is broken into submodules
- Every module has ONE job/operation

Software design can benefit from Least Privilege.

Modular programming breaks a program down into submodules.

Each module can have some least privilege applied to it.

Software becomes easier to:
- Read
- Reuse
- Maintain
- Troubleshoot

##### Non-admin accounts

Using nonadmin accounts, we can implement least privilege.

- Minimal set of rights
- Avoid a 'sysadmin' account existing (root, SA, admin, etc)
- Reduces risk

#### Separation of Duties

SoD means we NEED more than 1 person to complete a task.

- No single 1 person can fully abuse a system
- <https://en.wikipedia.org/wiki/Two-man_rule>
- Never give 1 person full control over a system
- Important in critical business areas
- Checks and balances

##### Software

- Common in software components
  - Ensure system checks and balances are performed
- Multiple conditions must be met before an operation can complete
  - i.e.
    - Does the user have permissions to invoke an operation?
    - (if the software is modular) Is the model requesting the operation allow to make that request?
    - Are the correct security protocols, like encrypted comms, in place?
- SoD in Software says that these checks and balances must be completed by different parts or modules
  within software, so each component is fully responsible for its own task
  - Each module has 1 job and must do it well, which minimizes risk to other components
- Code must be reviewed and tested to ensure each module performs properly
  - The code author MUST NOT review their own code. 
    This could allow them to insert malware into their code easily.
    - A different set of eyes will reduce bias and mistakes introduced by 1 person.

#### Defense in Depth

- One of the oldest security principles.
- Layering security controls to provide multiple defenses
- One single vulnerability will not result in a compromise
- Strong external network and a weak internal network...bad!
  - One hapless employee with a virus on their laptop can defeat the strong external defenses.
- Not just 1 strong firewall will protect you.
- Layers should be DIFFERENT.

##### Diversity

- Security layers should be heterogenous
- Mix protection
  - i.e. Input validation AND stored procs
- Wider range of security
- Deterrent and mitigation of risks
  - Effort to breach a system is a great way to make it a PITA to penetrate

#### Fail-Safe, aka Fail-Secure

- Systems should fail to a 'safe state'
  - A state that will not allow it to be compromised (at all, or further)
    - Don't do a memory dump!
    - Reboot > Login as Admin
  - Vehicle crash detection
    - Door unlocked
    - Engine stopped
  - Suppose a user attempts to log into a system
    - Bad password:
      - Error says "Login Failed" and not "Bad Password"
      <!-- - **It's Just Good InfoSec Bro &tm;**  -->
      - "Login Failed" is nondescript, IDK if the uname or pw is invalid.


- Rapid recovery upon failure
  - Failover server/module
    - i.e. <https://success.docker.com/article/dr-failover-strategy>
- Resiliency
  - Confidentiality
  - Integrity
  - Availability

- Fail-safe is part of the SD^3 initiative
  - Secure by design
  - Secure by deployment
  - Secure by default
    - \*Should be secured during every point in deployment
    - \*No default passwords
    - \*No extra default features

#### Economy of Mechanism

EoM is a phrase used when trying to implement functionality while keeping the implementation as simple as possible, but still trying to maintain the functionality.

- Usability vs Security
  - Generally opposing forces within an org or software system
  - Add a lock to a door on a room
    - Now everyone who needs access has extra steps
    - Takes time to lock/unlock the door
    - Admin duties now exist regarding assigning keys or changing the lock
    - Applying EoM to this example will have us use RFID cards instead of keys as they are
      easier to manage and are more convenient for users.
        - We still have the desired effect but the IMPLEMENTATION is different

Sometimes, more features are crappy hacks built on existing systems. This creates a more complex system that could hide security holes.

- Avoid unnecessary complexity
- KIS,S
- Operational ease of use with simplicity
- Fewer inconsistency

##### Requirements Traceability Matrix

EoM can be hard to understand and harder to implement.

A RTM can help you understand it.

- Generated during the requirements gathering phase of a project
- RTM is a document that tracks the requirements of software and
  matches it to the implementation components
  - This lets us compare what is being created and how it covers the requirements of the project
  - We can use this during the development phase to track and manage software functionality
  - Prevents inclusion of unnecessary functionality

Example: <http://doliquid.com/wp-content/uploads/2017/12/requirements-traceability-matrix-template-best-business-template-regarding-requirements-traceability-matrix-template.png>

#### Complete Mediation

- Access requests must be verified EVERY TIME someone accesses a system
  - Lots of systems actually don't do this

1.  Log into website
2.  Do something
3.  Must log in again
  1.  Each request is A&A (authenticated and authorized) individually
4.  This is a Gigantic PITA Bread

More practical approach is to use a Smart Card for authentication

- User needs to keep card inserted, not type `P@%$$w0rd` 2315 times.
- Authorization is NEVER circumvented
- Verify every single request
- This model enforces systemwide access control
  - When a user authenticates, the same authentication happens to the same 
    user account at each stage of the process
    - This means each component needs to use the same authentication mechanism.

This greatly reduces the possibility of a system exploit as any exploit would be forced to re-auth.

##### Caching

- Using CM is not common
  - jsessionid, anyone?


- Caching greatly speeds up software
  - Increase to security risk
    - Auth bypass
    - juicy session token gets stolen (session hijack)
    - MiTM
    - Replay attack
- The longer creds are cached, the greater this window of opportunity is.

#### Open Design

...AKA 

"security through obscurity S.U.C.K.S. and your crappy custom XOR 'encryption' protocol is a pile of duct tape and cardboard and would be torn to shreds by any hacker who found out it was being used"

Open design is the act of making a system and publicly releasing the source code.

- Depending on the secrecy of your design is a bad idea
  - Enables backdoors, poor testing, and shallow defenses

This enforces the idea that implementation details should be independent of design

- Looking at you, hardcoded password/server name/connection string/IP address
  - These would not be included in the design, and thus the shared source code

This permits craploads of people to review the software. And because there are no embedded passwords/ips/etc, the act of reviewing the software will not compromise any defenses.
- Public bug issues
  - Faster resolution
  - HOWEVER, public bugs become PUBLIC KNOWLEDGE immediately
    - 0days :3

Because of the bug problem, OD is not a universally accepted practice.

##### OD Crypto

Crypto is one of the best examples of OD in practice.

Crypto is Math that cannot have ANY flaws or it will crumble.

There are loads of crypto algs, some have been blown to bits (i.e. DES's keys are too short) but others are good (i.e. AES)

<https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle>

"A cryptosystem must be secure even if everything about the system except the key is known."

or,

"Assume the enemy knows the system".

This is an antithesis to "'security' through obscurity", which has been proven to breed awful \(in\)security.

Example: OD was NOT applied to the Content Scramble System to encode DVDs in the 90s.

<https://www.cs.cmu.edu/~dst/DeCSS/Kesden/>

They relied on a SECRET ALGO that was exploited because it was weak.

##### Moral

Do not rely on the mechanisms that you deploy to be secret.

Secrecy does NOT bring security.

Public scrutiny fixes issues faster than without.

#### Least Common Mechanism

Restricting multiple users from sharing the same "mechanism", i.e. a common component in a system.

If 2 users both access the same app on the same server, then the 2 users have multiple mechanisms in common like:

- Web server
- Application
- Network comms

This means one user can accidentally or maliciously access the private data of the other user.

LCM (Least Common Mechanism) refers to separating these environments as much as practically possible to separate data and controls.

- Sharing opens channels to transmit information
- To implement LCM principle, common mechanisms must not be shared
- Mechanisms that must be separated:
  - More than 1 user or process
  - Different levels of privilege

Example:

1 normal user, 1 manager user.

If we add mgmt functionality to an app that both users use, then the normal user might gain mgmt functions.

If we apply the PoLCM (principle of LCM), then we might want to make 2 different applications.

Session hijacking highlights why the PoLCM is important: It would mitigate it greatly.

- Web server is shared
- Network is shared
- Hundreds of users pass a session token back and forth
- Might be admins/mgrs using the same web server as normal users

#### Psychological Acceptability

Psychological Acceptability is abut recognizing that humans are involved when working with computer security.

This can be hard as all people behave differently.

Ex: Long and complex passwords are more likely to be written down near the computer, and therefore this requirement, while technically secure,
may effectively decrease password security, and piss off users.

Security mechanisms should not make resources more difficult to access. Each layer of difficulty will only encourage users to circumvent them.

Security mechanisms should be transparent, but are rarely transparent.

Complexity of configuration also may lead to insecure software. The harder it is to configure software, the easier it is to misconfigure.
- Configuration should be as easy and intuitive as possible.

Outputs must provide understandable errors.
- No privileged information should be given.
  - 'incorrect creds' vs 'incorrect password'
- Properly described incorrect parameters or errors.

#### Weakest Link

The Weakest Link is the most easily compromised point of a piece of software.

The WL (Weakest Link) is how resilient the software is against a hacker.

The hardest part is actually identifying the WL. Many admins who respond to breaches had no idea the hole existed in the first place. They probably don't monitor or audit either.

It's also important to consider what results in the LARGEST vulnerability (a combination of scope of impact once breached PLUS ease of performance of vulnerability)

- What software components could be breached?
  - Code
  - Services
  - Interfaces

A common mistake orgs make is ONLY focusing on user interfaces and ignoring other possible routes of exploit, like backend services or hackers editing code.

ANY type of break in the weakest link means a breach. WL is a component that CANNOT be compromised -- Some can, and don't impact much of other systems.

#### Leveraging Existing Components

Adding new functionality or writing new code can introduce security vulnerabilities.

Existing components should be used/reused to ensure attack surfaces are not increased, assuming the existing components have already been audited for vulnerabilities.

Q: Do we introduce the functionality as a separate component, or as a change to an existing component?

A: We need to balance EoM (Economy of Mechanism), which is about keeping things simple, with adding new functionality as a new component.

If adding new functionality to the system is MORE COMPLEX than modifying an existing component, we should do the simplest thing! And vice versa.

Keep in mind that any changes to an existing component should be audited.

For example, databases should be leveraged instead of rewriting the database system.

As always, changes bring security issues, and pro/con assessments must be made.

### Privacy

#### The Privacy Principle

Privacy is about controlling the information about something -- Allowing the user to control how information is shared.

This is an important topic and often controlled by law.

- Who is it shared with?
- Why?
- How will it be transferred or used by the 3rd party?

"Traceable sharing" is a way for a user to share information with another party, but it possible to track where the information was divulged.

- ex: Credit card purchase on a site
- We can also verify it is used correctly

Unfortunately, a lot of the time, we can't know how the third party is using the data -- It isn't traceable. We are just trusting them.

##### Data Disposition

Data Disposition is the long-term use of data.

ex: Credit card purchase.
- Do we retain their address? CC#?
  - We are obligated to administer their data according to our policies

If the data that we store is compromised, we will become liable, or lose customers (or piss them off!)

*cough* EquiFax *cough*

Also, using customer data in a test environment is a bad idea:
- Discloses it to all your devs
- Duplicates it across an entirely new system, 'doubling risk'
- Test environments are generally less secure
- Data should be anonymized first
  - Name, address, CC# should be replaced with random data.

#### Privacy Considerations/Privacy Policy

A Privacy Policy is a high-level document that details the following about private information:

- Collection
- Use
- Transfer
- Storage

This document is used to identify what information needs to be safeguarded, how, and the details.

It can also be a guide for employees.

Part of a Privacy Policy is to have a Privacy Disclosure Statement, a public version of the privacy policy, so external parties can understand how data is used and protected.

##### Identifiable Information/PII

Identifiable Information (II/PII) is information that could be used to identify a person.

- Name
- DoB
- Birthplace
- Address
- TIN/SSN/NIN
- Motor Vehicle/Driver's License
- Genetic info/face/prints
- IP Address

It doesn't take a lot of info for data to be considered PII.

###### Protected Health Information

(PHI)

- Demographic data
- Biometrics
- Medical History
- Test data

This area of data is protected by HIPAA/HITECH Acts.

Storing info under this category MUST be protected according to legal mandates.

##### Breaches

Since you are storing PII, you must monitor for breaches.

Lots of companies only take action AFTER a breach is detected, or even worse, never find out a breach occurred because there was so little monitoring.

Security controls must be put in place to:

- Detect a breach
  - What happened
  - How did it happen
  - What data was compromised
  - Will involve logging and auditing

Data should be encrypted so that data cannot be read w/o decryption, which could take years or more.

Some legislation dictates long term data protection guidelines.

#### Protection Principles

In Europe, the EUDPD, or European Union Data Protection Directive, dictates how data is protected.

- Any data collected can only be used for approved purposes as dictated by the owner of the data
- Destroyed after a period of time or rendered nonidentifiable
- Data should not be processed, unless:
  - Usage is transparent (user gives consent to processing)
  - Purpose of processing is legitimate
  - The level of data collected is proportional (appropriate) to its purpose.
    - Helps stop orgs from collecting unnecessary data

#### Safe Harbor

Due to the EU/US differences in data laws, a set of "Safe Harbor" rules were made.

Data can be transferred from Europe -> US, GIVEN THAT:
- Notice
  - Customers must be informed of how their data is collected and used
- Choice
  - Customers can opt out of the transfer or sharing if they choose to
- Onward Transfer
  - Data can only be transferred to third parties who follow data protection principles
- Security
  - Reasonable efforts must be made to protect the data
- Data Integrity
  - Only the data that is required is transferred, and it's used for the purpose it was collected for.
- Access
  - Customers can access their information and correct/delete it.
- Enforcement
  - There's an effective way to enforce these principles

### Risk, Governance, and Compliance

#### Regulations and Compliance

##### Federal Information Security Management Act (FISMA)

This act governs the security of Federal Information Systems and ensures that periodic risk assessments are completed.

- Policies and procedures are in place to mitigate assessed risks
- All subordinate levels (facility access, information systems, etc.) have appropriate security planning
- Staff are properly trained
- Policies are periodically tested and evaluated to ensure their correctness
- Remedial processes are in place to...
  - Implement, document, and evaluate remedial actions
- Planning is done
  - Disaster recovery
  - Etc

###### Identification

Classifying data is IMPORTANT to apply the right level of security to data.

FISMA asked NIST to determine the standards for classifying data within federal agencies.

- Standards
- Guidelines that govern what classification is applied to what data
- Minimum security requirements for each classification

Result of this: NIST made FIPS 199, FIPS 200 publications.

<https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.199.pdf>

<https://csrc.nist.gov/csrc/media/publications/fips/200/final/documents/fips-200-final-march.pdf>

##### Sarbanes-Oxley (SOX)

SOX is an act that governs corporate financial practices in reporting.

- Created in 2002 after a bunch of corporate financial scandals
- Includes:
  - Public Company Accounting Oversight Boards
  - Enhanced Financial Disclosure and Reporting
  - Corporate and Criminal Fraud Accountability
  - Corporate Tax Returns

SOX includes requirements for recordkeeping and reporting of financial data.

Any system that maintains this type of data must comply with SOX.

##### Health Insurance Portability and Accountability Act (HIPAA)

- Governs the security and privacy of health information

It also governs any org that collects health information.

- Final Security Rule
  - Categories:
    - Protection of transmitted data
    - Data at rest
    - Physical protection
    - Administrative procedures
  - Standards
    - Administrative
    - Technical
    - Physical safeguards
- Final Privacy Rule:
  - An entity (like a healthcare provider) can use any information disclosed to it for its own treatment, payment, and healthcare operations within the organization.
  - Covers issues like notification to patients, prohibiting sale of PII, passing health info to researchers (and how), etc.

##### PCI Data Security Standard (PCI DSS)/Payment Processing

Payment processing is also regulated, through a standard called the PCI DSS.

This standard is used by the PCI (Visa, Mastercard) for the protection of cardholder data and transaction protection when processing transactions like credit card payments

The PCI DSS is broken down into a lot of areas that govern building and maintaining secure networks.
- Build and maintain secure networks/systems
- Protect cardholder data
- Maintain a vulnerability management program
  - Antivirus
- Implement strong access control measures
- Regularly monitor and test networks
- Maintain an Information Security Policy

The DSS is applied to all parties involved in processing.

- Merchants
- Processors
- Acquirers
- Card issuers
- Service providers
- Anyone else who stores, processes, transmits cardholder data.

Credit card fraud is very common, so these regulations make sense.

#### Legal

##### Patents

A patent is a set of exclusive rights granted by a government to an inventor or assignee.

These rights are granted for a specific period of time and are to protect the inventor's rights, so noone else can claim it was their invention like selling it w/o the patent owner's permission.

Patent law varies between countries and can be complex.
- What is patentable?
- How long can it exist?

To get a patent, the owner of the work will need to apply for a patent with the government.

##### Copyrighting

This is another way to protect IP rights.

- Gives a creator of original work the exclusive rights to it.
- Copyrights are applied to any expressible form of an idea or information that is substantive and discrete.
  - Covers
    - Creative
    - Intellectual
    - Artistic

Different from a patent because patents usually don't cover ideas, but focus more on inventions.

Copyrights are governed internationally through the Berne convention.
- Countries that are a part of the convention must recognize each other's copyrights

Depending on the country, copyrights are automatic or must be applied for.

For countries part of the Berne convention, copyrights are automatic. However, you must be able to prove that the work was created by the owner and WHEN it was created.

If an owner of a work thinks their work was copied, in most countries, it is up to the owner to pursue it in court.

##### Trademarks

A trademark is an identifiable quality used to identify products or services from an organization.

This is usually used to protect an org's brand. Logos are usually trademarked.

Trademarks can be made via...
- Common law
- Registration
  - Registration provides the owner a lot more legal protection and the ability to recover more damages if they are incurred

Internationally, trademarks are managed through the World Intellectual Property Organization.

##### Trade Secrets

These protect a secret for a specific amount of time.

- Very tightly controlled
- Offer a lot of protection
- Usually food
  - ...szechuan sauce!!!!
  - Coca Cola

This is difficult to use in Software though: If another party develops similar software INDEPENDENTLY, it is no longer a trade secret.
Example: Using a shopping cart on an ecommerce site.

##### Warranty

This is a protection of a consumer that ensures a product or service will work as advertised, or else they are reimbursed in some way.

- Minimum legal protection for consumers
- Protection:
  - Quality
    - No poor quality
  - Safety
  - Performance (performs as advertised)
  - Durability
  - Defects

Software generally comes with NO warranties...

This means that if you buy software with security holes, there may be no recourse for the consumer to pursue the software retailer for damages...

#### Standards

Standards are not necessarily the most common way of doing something.

Standards are a defined level of activity, usually measured by a third party! A third party must be able to say if you meet a standard or not.

Benefits:
- Can compare 2 organizations
- Can promote interoperability

##### International Organization for Standardization (ISO)

- Founded in 1947

ISO develops and publishes international standards that ensure products are safe, quality, and reliable.

There are over 19,500 standards published for tech, food, and health.

##### National Institute of Standards and Technology (NIST)

- US based
- Develops tech, measurements, and standards that align with the US economy
- Federal Information Processing Standards (FIPS)
  - Governs all nonmilitary gov't agencies and gov't contractors when specified as part of their contract
  - Crypto
  - Personal ident verification for feds + contractors
  - NIST SP 800 series
    - Research and guidelines on securing information systems
  - SAFECode
    - Non-profit, not related to NIST
    - Identify and promote best practices in software dev

#### Risk Mgmt

- Many models exist to model risk
- We'll look at a general model

Risk mgmt's goal is to assess and mitigate anything that could cause harm to the project or deliverables.

- Manage risk in general
- Manage risk through project phases

##### Step 1: Asset Identification

This is where we identify and classify all assets.

- Identify and classify
  - Assets
  - Systems
  - Processes

A common mistake is to only focus on assets we want to protect instead of ALL of our assets. When we do this, we miss protecting "noncritical" assets whose compromise may lead to larger issues later on.

Risks can be weighed by considering:
- Damage to business
- Damage to people
- Financial risks
- Etc

Scoring each factor helps you objectively determine where your priorities should be.

Then, prioritize assets.

Financial costs (lawsuits included) generally are weighted more.

We also need to evaluate the information criticality of the data. This refers to how critical the data is to the business.

For example, if someone loses access to some data, does the rest of business stop?

##### Step 2: Threat Assessment

- Identify threats
- Identify vulnerabilities
  - Exploitable vulns can be used to gain access to all sorts of things, so focus on them.
  - Vulns can climb 'priority ladders' to gain access to higher priority assets
    - Risk priorities must reflect this
- Threat = harm to an asset
  - 'Threat' includes...
    - Incorrect data entry
    - Insider threat

##### Step 3: Impact Determination and Quantification

What's is impact the loss/compromise of an asset would have?

- Determination of loss
  - Tangible (easier to quantify)
    - Financial loss
    - Physical damage
  - Intangible (harder to quantify, but do it anyways)
    - Reputation damage

##### Step 4: Control Design and Evaluation

What controls are needed to mitigate the risks and/or reduce the vulnerabilities?

Controls are countermeasures introduced to reduce/eliminate the risks.

- Controls
  - Actions
  - Devices
  - Procedures

They can be additional action that occur, physical/logical devices, or procedures that address specific risks.

For example, a website that lets you buy stuff with your credit card has the risk of credit card fraud.

One countermeasure could be additional security checks to mitigate that risk.

##### Step 5: Residual Risk Management

This step exists because risk cannot be fully eliminated from a system, and we need to accept that fact.

These remaining risks are called 'residual risks'.

We need to understand these risks to identify and introduce controls along the way to reduce these risks.

- Consider multiple controls to reduce risk

### Software Development Methodologies

#### Waterfall

```

Requirements \
             v
            Design \
                   v
                  Implementation \
                                 v
                                Verification \
                                             v
                                            Maintenance
```

This methodology is the first and most common development methodology.

It's based on a sequential set of phases that govern what's done in each phase.

It is the most used because it's the best known (and old AF.)

It came from the manufacturing process and is simple but NOT adaptive, which is a problem for software projects as requirements are known to change a lot.

Because of this, we need to repeat a lot of work to introduce new requirements.

Waterfalls therefore have the highest risk for increased time and cost.

##### Phase 1: Requirements

All of the requirements of the software are determined and presented as a deliverable.

Requirements are either:
- Functional
  - Describes the function of the system
- Nonfunctional
  - Describes the rest of the requirements that are related but don't fall under the functionality of the system
  - Restrictions are an example of a nonfunctional requirement

These requirements are passed to the next phase.

##### Phase 2: Design

In this phase, software architects blueprint the design of the software.

The "External Functionality" is how the software interacts with the outside world.

- External Functionality
  - Input
  - Output
  - Constraints

- Internal design
  - Algorithms
  - Data structure
  - Internal Routines

At the end of this phase, you should have clear documentation, which is the blueprint to move to the third phase.

##### Phase 3: Implementation

Documentation is used in this phase to start making the software. The software should not be difficult to create if the architects have done their job. The architects' documentation must be clear and concise

Unit testing should be introduced here as well, even though it is technically part of Verification.

At the end of this phase, software is done being created.

##### Phase 4: Verification

- Software is tested for bugs and errors in this phase
- Devs fix the software
- This repeats until bugs/errors are minimized

In a waterfall method, devs generally have to introduce A LOT OF CHANGES in this step. You can't go back, and all these changes have to be retested again.

Verification is the cycle that causes the majority of cost and time overruns:

    Step 4 -> A BUG! -> 1 -> 2 -> 3 -> 4 -> here we go again...

At the end of this phase, software is deployed.

##### Phase 5: Maintenance

The software has been deployed into PROD at this point. Hope that `drop from dbname` test that runs post-update was behind a conditional! ;)

Focus of this phase is to keep software up and running.

- Correct any unresolved or undetected bugs
- Fix the bugs
- Routine maintenance (backup, recovery, etc)
- This phase is active until the software is retired 100%.

This phase is the longest phase and is usually most expensive in terms of time and money.

However, most orgs don't include these numbers as part of the project. This is attributed to the cost of RUNNING the software and not the development, so the Implementation phase is usually recognized as the most costly and time-consuming phase.

The architect and developers should be creating software that is easy to maintain because the last 2 phases are so time and money consuming.

#### Agile

The Agile development methodology is a group of dev methodologies that share 'agility'.

When done right, agile methodologies are much better suited for software projects as they align with the short, iterative style of agile.

Software dev is always a work in motion and not a static, monolithic piece of work.

- Promotes flexible response to changes
- Adaptive Planning
- Teamwork
- Collaboration
- Adaptability
- Development

##### Scrum

<https://www.scrum.org/resources/scrum-framework-poster>

Scrum is a popular Agile dev methodology.

Focused on programming, and a 30-day release cycle. Even if the software isn't ready for market, it's maintained in a deliverable-ready state, which helps to test it.

- Simplified change mgmt
  - This makes Scrum unique.
  - Scrum focuses on sprints (30day) and scrums (24hr)

Sprints maintain goals over a 30day period, and the daily scrum serves as a way to manage progress per-day, so that everyone has daily goals.

Scrum also has processes to evaluate each sprint and introduce work back into the cycle.

If a bug is detected, it's added to a 'product backlog' and eventually picked up by a sprint to fix.

Scrum is popular because of its simplicity and clear understanding of progress being made during the dev cycle as small changes are tracked.

##### Extreme Programming (XP)

```

      Planning -------------- Iteration
          \                      /
           \                    /
            \                  /
             \                /
              \- Acceptance -/

```

This methodology focuses on 'user stories' and is useful for smaller dev teams.

'User stories' are a way of describing a user's interaction with the software as they do a task. This helps provide requirements of software.

These stories are handled by the team to finish the project with small changes.

The iterative model in XP follows a feedback model that allows for feedback and acceptance to repeatedly come back into planning.

In the real world, a lot of orgs lack experience and understanding to properly implement Agile dev methodologies, fail, and return back to waterfall.

Agile has significant advantages over waterfalls, so understanding the methodology is important so that you know if it's being implemented correctly.

## Secure Software Requirements

These integrate security into the software dev lifecycle.

### Policy Decomposition

Policy decomposition is one of the key components for aligning policy statements.

Security policies are composed of both internal and external requirements.

#### Internal Requirements

- Proper protection for audit/data logs
  - Logs contain a lot of important information
    - Account names
    - Server names
    - Names/IDs of processes
  - Audit and data logs must NOT be tampered with
    - This is to ensure non-repudiation
      - We can ensure that one individual performed a specific action
      - If someone tampers with an audit log, we can no longer ensure that any specific individual performed a specific action
  - Data loss!
    - Data loss usually comes from IMPROPER HANDLING, not data loss

Every security policy needs to ensure data is handled properly.


- Managing Data Loss Prevention (DLP) elements:
  - Governance policies
    - Data is handled properly by the correct people 
  - Risk assessment
    - How likely is it for data loss to occur?
      - What happens when data loss occurs? 
    - Rank threats, likelihood, and response to a threat occurring
      - Create appropriate response plan, don't over/under-respond
  - Compliance
    - Compliance with DLP means we comply with business/legal requirements with respect to data
      - i.e. legal req to retain 7 years worth of data...The security policy needs to ensure we meet these legal obligations for the lifetime of the data
  - Classification
    - Data is properly identified so we can put the correct security controls around the correct data
      - i.e. CC# or PII should be more protected than a product catalog
  - Policies
    - Governance and compliance are key factors in determining an organization's policies
    - The high-level policies of an org are generally made up of (or mapped to) a lot of smaller, simpler policies
    - Policies such as 'who can modify data' can affect the integrity of that data
  - Discovery
    - Data within an org gets copied, duplicated, and edited very fast
    - Discovery allows us to find data in an org that needs to be protected
  - Remediation
    - Sooner or later, we will not be compliant with policies
    - Remediation lets us determine the course of action to take
    - i.e. if we discover someone gets unauthorized access, we review the audit logs to determine if they actually used that privilege.
  - Awareness
    - Everyone enforcing security policies needs to be aware of the policies that they need to comply with
    - We also must be aware of whether or not we are compliant with the policies
    - Monitor internal system traffic, and internal controls, etc.
      - Collect evidence of whether or not we meet the requirements of our policies
      - What system traffic/controls do we monitor?

#### External Requirements

These address all of the security issues/requirements that are outside of our organization, like external vendors, gov't, and even WFH (Work From Home) employees.

- Protect all external connections
  - Security controls
    - Enforce compliance upon connections
      - i.e. all extern access must be done via VPN
    - Our security policy must dictate what types of connections are allowed, who is allowed to use what type of connection, and details of security requirements of each connection, like the level of encryption.
    - Once an external connection is made, access must be restricted only to the services and components that that person has permissions to access.
  - Authentication
    - Minimum levels of auth and what protocols we support
      - This may bar devices from connection, which may or may not be acceptable.
    - Account policies to protect against things like bruteforce

How do we mitigate web-based threats?

When a client/employee, who SHOULD be listening in that morning meeting, goes to hxxps://www.mycoolguns.ru/, they're downloading HTML/CSS/JS/etc from that site and bringing it to an internal website.

The security policy has to address the requirements to do this safely and within policies.

- Content filtering
  - Restrict known dangerous sites (blacklist)
  - Could use a whitelist (can be very restrictive)
  - Could block specific filetypes (.exe)
- Proxies 
  - An external computer (mycoolguns.ru) should never have direct access to an internal computer (Bob's laptop)
  - Proxies hide a client's connection to a webserver by forwarding requests...mycoolguns.ru only sees the proxy IP and whatever else the proxy chooses to forward/not forward
  - The external webserver will never have direct contact with the client

### Data Classification and Categorization

These determine how data will be handled during the development process.

#### Data Classification Part I

One of the most important aspects of building a secure system is how data is classified.

Data is the key asset of an enterprise. All systems around it are there to manage it. Without data, how can money be made, or actions be executed?

- Enterprise data flows throughout an organization
  - Data flows are complex
  - Maintenance and management can be difficult

Data flows through enterprise. We need to know how so we can put security measures in the right places.

- How does data come in
- How is it stored
- How does it move between internal and external systems

If data enters via a website, the way the CC# is communicated is likely different than how we communicate that info to a bank for authorization, which means the security requirements will be different for those 2 scenarios.

- To help manage complexity of data flows
  - Data is classified and labeled
  - Responsibility for management is identified

We must identify data that has the same security requirements: SSN, CC#, etc.

- Data can be classified by its:
  - State
    - How data flows
    - CC# needs to be used to authorize a purchase, and communicated to a bank
  - Use
    - i.e. CC# needs to be transmitted over this medium
  - Risk impact level
    - What does it mean to our org if this datum were compromised, and what is the possibility of that happening?
    - What does it mean if this (or ALL) CC# were leaked?
      - Short-term business
      - Long-term business
      - Legal implications
    - By identifying both risk level and impact level, we can focus on the most important thing. We would want to protect the most high-risk, high-impact data as opposed to low-impact, low-risk data, etc.

- Data can exist in any state at any time, like/because of/in the form of:
  - Storage
  - Creation
  - Transmission
  - Modification

These are all important aspects as data can be spread across so many different systems (physical/logical/network) because of this.

- Data location/hosting is IMPORTANT
  - Physical, permenant media
    - Hard Drive/CD,DVD
  - Remote media
    - USB, cloud/hosted storage
  - RAM
  - These all impact security requirements. Policies for portable device encryption? Hacked systems java mem dump? Etc.

#### Data Classification Part II

- Classification of data can also be determined by its usage
  - How does data align with business needs?
  - How shareable is it?

CC#:
  - Who needs to read or edit it?
    - This determines who has perms to do it and when they can do it

Data with the same shareability and business needs should be classified together.

- Is the data:
  - Internal
    - It has been created and used WITHIN an application
    - Generally hidden by a system
  - Input
    - This data is read INTO a system
    - Classified as input data
    - Data can be given by a user or third party
  - Output
    - Generated by an app after being processed
    - i.e. a report
- Additional attributes:
  - Security-sensitive
    - This is data that is of high value to an attacker
    - This category is NOT the same as high-risk...
  - Personally Identifiable Information (PII)
    - CC#, TIN, SSN
    - Address, phone number, IP
    - These are protected by LAWS, and care must be exercised.
    - Customer/employee (*cough* EquiFax *cough*) leaks can destroy lots of parts of our business
  - Should be hidden
    - Any data we wish to conceal from the public or users
    - i.e. a database connection string used by an app to connect to its DB.

#### Data Ownership

Who owns the data?

- Data is owned by the organization
- To assign responsibility, we think that data is 'owned' by specific employees
- The owner is responsible for the data.
- Data could be assigned to users who are considered to be stewards of the data
  - A steward takes care of the data from day-to-day
- Role determination is dependent on the type of data as it relates to business needs


- Data owner
  - Determines who has access to the data and what level
    - R/W, change, delete
  - Defines:
    - Classification, authorized users, access rules, security controls
- Data custodian
  - Directly interacts with data
    - Responsible for fulfilling the data owner's requirements
  - Maintains:
    - Security controls, Access controls, authorized users, data backup; retention; and disposal.
  - Data custodian has many responsibilities, you will see many organizations separate these responsibilities among multiple custodians.
    - Custodian A does backup
    - Custodian B manages authorized users and their perms

#### Labeling

Labeling data is about adding extra data (metadata) to describe the data that we're protecting.

Example of the importance of labeling is a file called `README.txt`. What's the file about? What info does it have? Who should have access? Does it contain confidential info? Is it an original or a copy?

- Enterprise data retention can be extensive, depending on policy.
  - How do we know something must legally be retained?
- Difficulty is identifying which data must be retained, and by what policy.
- Appropriate data labeling ensures that proper handling is performed
- Data sensitivity can be defined by job role and/or business reasons for the data existing
  - If an employee has a legit business need to use the data, they should be given access


- Improperly handled data and its impact organizationally is MORE IMPORTANT than data sensitivity
- All data should be classified by its impact
  - High
  - Medium
  - Low
- Delineation determined by impact to...
  - People
  - Customers
  - Financials

Example: Flight data for large airliners is transmitted WIRELESSLY to the flight computers onboard the airplane.

Any improper handling of this wirelessly-transmitted data could compromise the flight systems onboard the airplane.

The impact could be ~80 dead people, so pretty high. You'd also lose a lot of customers and lots of money.

This data is High, and must always be protected, never mishandled.

#### Data Types

##### Structured

Examples:

Charts:

<https://www.bls.gov/cps/employment-situation-covid19-faq-june-2020.pdf>

MS Access databases:

<https://docs.microsoft.com/en-us/dotnet/framework/data/adonet/sql/linq/downloading-sample-databases>

The data fields are separated by special delimiters:

| id | fname | lname | address         | phone        |
|----|-------|-------|-----------------|--------------|
| 0  | henry | post  | 123 example ave | 123-456-7890 |

...etc...

Data is also restricted in some way, i.e. the `id` field can only accept numbers, maybe the `phone` field can only accept hypenated numbers, etc.

<a href="/feed.xml">Structured data could also be XML files.</a>


This one looks like:

```
<feed>
  <entry>CONTENT...</entry>
  <entry>CONTENT...</entry>
  <entry>CONTENT...</entry>
  ...
</feed>
```

##### Unstructured

This data is (probably) data you're the most familiar with.

<a href="/hobbies/">Click this for an example of unstructured data.</a>

Emails, word documents, reports, essays, letters, ...paintings, etc. are all unstructured data. That machines do not know how to process. You can write whatever you want in an email or essay, especially a word document, ESPECIALLY a powerpoint.

There are no restrictions or requirements in these media.

#### Data Lifecycle

All data has a lifecycle.

Created, used, stored, destroyed.

All data follows this pattern, and your data (yes, even SSNs and CC#) will go through this cycle.

When data is created, how does confidential data move between a customer and our organization securely? During the time we have the info, we must protect it, and it is also our organization's LIABILITY.

This means we also have to DESTROY it, if we do not need it. We should, too.

- Data lifecycle mgmt is a data owner's responsibility.
  - Also responsibility to define level of importance and retention that a set of data should have
  - Retention impacts the data lifecycle 
    - A policy that requires 7 years retention means we have to be responsible for the data for at least 7 years
  - It is possible for data to pass from one owner to another in complex scenarios


- Data lifecycle can be broken into:
  - Data generation
    - Importance lies in persistent data
    - Classification, protection, and destruction policies are assigned. Very important.
  - Data retention
    - Data owner and custodian responsibilities are defined
    - Protection levels and duration of storage are defined
    - Stewards and owners are responsible for the data now
    - Not only logical, but physical measures (backups, etc) should be enforced to keep in compliance with policy
    - How many different places are our data stored? Backup!!! Also, compliant (maybe encrypted) backups!
  - Data disposal
    - Conserves resources
    - At EOL, data custodian removes data from ALL sources.
    - Failure to observe this may cause us to forget about the data and cause it to be compromised.
    - It is common for old data to be forgotten about, mishandled, and then compromised, leading to negative effects.
    - When disposing of data, ALL SOURCES that contain it must be dealt with, and this includes backups.

### Functional Requirements

Functional Requirements define how software is expected to operate.

#### Role and User Definitions

- Review roles and user definitions, and how they relate to functional requirements

When making software, one of the first criteria is:

- Functional software 
  - (it works)
  - Meets all of the functional requirements

- Functional requirements define the expectation of software functionality
  - They are driven by business requirements

- We may be building software that multiple business units use, and therefore they may have different requirements. Might conflict, even.

However, all organizational requirements are ultimately used to create the functional requirements our software needs.

Once we know what our software will do, we can then understand the roles and users that are allowed to use those functions.

This way, the right people get access to the right functions.

- Role and user definitions will outline who does what
  - What group can use a system
  - Job roles further define specific functions
  - It's common to see role definitions line up with org job definitions
- A comprehensive listing of users and their functions outline part of a 'use-case' definition
  - Use cases are part of the UML standards
  - It's a diagram that helps define each function within a system and who (users aka actors) can interact with those functions
    - It also describes the details of a function and what it does
- Users are called 'subjects' in comp sci terms (subject-object matrix)

#### Deployment Environment

- We need to consider other things that are maintained to create software, such as the development environment, test environment, and deployment environment.

Deployment env can be complex as software generally communicates with one or more other systems, and relies on core components.

This env is not straightforward.

The software dev lifecycle should follow the SD Cubed principles, or SD3.

- Secure by design
- Secure by default
- Secure by deployment
  - Software that's the first 2, but not secure by deployment, can be compromised DURING deployment. Pbllllt.

Software deployment in the environment relies on conforming to:
- Maintainability
- Data access
- Access to essential services
  - Auth like active directory

- Functional requirements for system deployment must be detailed
- Defining, understanding, and conforming to all infrastructure requirements allow for:
  - Seamless interconnectivity between disparate systems
- Example of infrastructure requirement is that our org uses IPv4. Any comms must use IPv4 and comply with this infra requirement.

##### Requirements Traceability Matrix (RTM)

![](/images/2020-06-15-CSSLP-notes/requirements-tracebility-matrix-example.png)

An RTM is used to track and manage all functional requirements as well as implementation details.

It also lists validation criteria, which are important b/c they let us ensure we're meeting those requirements.

RTM is really a documentation tool to help manage a list of requirements that helps us ensure every requirement is met, and prove that they are met.

#### Objects, Activities, and Actions

Objects are anything a SUBJECT can interact with when operating a system:
- File
- Database record
- Program element
- The system itself

We need to understand these objects because we must determine and control who has access to them, what functions objects allow, and who can access the function.

- Object access is controlled by implementing access control lists (ACLs)

Example: We have at a file. We need to control who has access to the file. Reading it, editing it, etc, need to be controlled.

Disk drives are also examples of objects. Perhaps a disk drive has a 'format' action that a user can access, even though they cannot access files INSIDE the disk drive.

Another example of an object is a user account. Some people need to CRUD user accounts. You can also reset passwords and add accounts to permission groups

Because of this, it is important to define all of the objects and their functions so we can control and secure a system.

One way to do this is to use a subject-object-activity matrix. This document outlines who has access to what objects and the activity they are allowed to invoke.

##### Activities and Actions

When we list all objects in a system, we must understand the actions and activities that a SUBJECT could do to each object.

- Activities and actions are allowable events that a subject can do to an object
- Specific activities are defined by the OBJECT, i.e.
  - File: access, modify, delete
  - DB record: create, read, update, delete
  - Disk drive: mount, unmount, format
- All activities and actions should be defined and documented
  - This is to ensure that these activities/actions are PROTECTED PROPERLY.
  - *any undocumented or overlooked functionality could be used to compromise a system*

#### Sequencing and Timing Requirements

Sequencing and timing can definitely affect applications, especially now.

- Disparate systems can attempt SIMULTANEOUS interaction with an object
  - Esp. servlets or web requests
- Also, different systems can try to access the same OBJECT simultaneously.

A number of problems could arise from this simultaneous access.

Events can also occur out of order due to thread timing differences if two programs are running.

When a triggering event happens, if TWO threads react to it, they may do the same thing with DIFFERENT timings.

If there is a dependency between two threads, i.e. Thread A needs Thread B to do Action X, say, to prepare data, before Thread A can do Action Y, to use the data, then these threads MUST work together and have a mechanism to do this.

This is important so that actions do not happen prematurely.

Issues that can arise:
- Race conditions and infinite loops
  - This impacts data activity design and implementation

##### Race conditions

- Software flaw
- Difficult to predict and find
- Can cause complex logic loops (no, using `Thread.sleep(2000)` will not mitigate this!!! STOP USING DELAYS! AAAAAAARRRGHHH!!!)
- Mitigated via Mutex locks
  - TBH, any locking mechanism will mitigate race conditions

These occur when 2 processes need to access the same object.  

Mutex is essentially a way to allow one thread to lock an object for ITS EXCLUSIVE USE and DENY any other thread access to the object.

If mutex is not properly handled, it is a software bug and may return an error or crash. Or do other horrible things. If this is the case, errors only happen during a small time window and therefore are very hard to actually encounter reliably.

If the two processes are separated by a lot of time, the error PROBABLY will not happen, so it APPEARS that the system works. But it will fail from time to time.

This failure frequency can depend on:

- Length of a time a process accesses an object exclusively,
- How frequently an object is accessed
  - By extension, the load on a system

Databases are full of locking mechanisms. ACID! Thread 1 marks a DB as 'in use', and Thread 2  waits. 

##### Infinite loops

- Occur with complex logic when loop controls fail
- Application can become unresponsive
- Mitigated via proper loop controls

To avoid this, the LOOP CONTROL must be implemented properly.

For complex conditionals, like `((if A ) or ((if B) and (if C) xx ...))`, developers must take into consideration ALL POSSIBLE STATES, not just common or expected ones.

### Operational Requirements

#### Deployment Requirements

Software is usually connected to other programs, and rarely runs on its own.

When outlining deployment environments that are necessary to install and configure a software, we need to outline all activities and tasks that need to be done by different people during the deployment.

Deployment requirements outline activities and tasks to be done by the software:
- Acquirer
  - Purchases the software
- Supplier
  - Supplies the software
- Developer
  - May customize software during deployment
- Or any combination of roles

All of these deployment requirements must be outlines in a plan that not only details who does what, but also when each task needs to be completed.

- Plan is executed by fulfilling the requirements based on SOP outlined by the organization

The best way to test the Dep Reqs is to repeat these activities in the TEST environment, and is also a good way to reset the test env to the beginning by redeploying over and over.

- The deployment plan should indicate a method (problem reporting) to capture real-time performance data for:
  - Process mgmt
  - Process improvement

One important part of a deployment is being able to identify when things DO NOT go as planned.

When something goes wrong, how do we know about it?

Every deployment plan must have a way to identify if a problem is occurring.

Ideally data is captured realtime, and issues are identified as they are occurring, so that potential compromise windows are as small as possible.

#### Operations Requirements

Once software is deployed, we need to understand and docuemnt the operatioal requirements in order to run it day to day.

- Make sure the product is effective in its intended environment
- We must know if the software is running effectively or not
- We need to involve many different people in our organizations
  - End users
  - Admins
  - Etc

- Overall goal is to define and incorporate an established set of activities and tasks to:
  - Ensure proper system operation
  - Ensure proper software product operation
  - Ensure proper software service operation


There should be a schedule for all of these operations tasks and when they should be executed.

- Established set of activies and tasks are performed daily across the organization to:
  - Monitor the operation of software/systems
    - SLA
  - Identify and escalate all abnormal incidents
  - Document and track all user problems
  - Escalate unresolved problems via a resolution process
    - All problems should be escalated UNTIL they are resolved
  - Initiate accepted stakeholder problem resolution
    - Once accepted, they must be implemented
  - Verify and validate accepted product change
    - We must ensure that the change WORKS/fixes the problem.

#### Management Requirements

Mgmt Reqs =/= Ops Reqs. 

Ops Reqs is focused on DAY TO DAY, while Mgmt Reqs are focused on ensuring the software+env are properly maintained.

##### Managing incidents

- Managing Incidents
  - an Incident is any event that interrupts the TYPICAL FUNCTION of a system/software.
    - i.e. my banking site is down, or I can't view my balance, etc.
    - OR, "my work laptop doesn't turn on!"
- Involves:
  - Monitoring incidents
  - Analyze incidents that do occur
  - Appropriate response is made

The docuemtnation and process are the most imortant parts.

If we had an incident and the admin fixes it w/o documenting it, did it happen at all? How can we prove it? (if a tree falls...) How do we know what was done to fix it? How do we know if it was the right way to fix it? How can it be fixed when that admin gets hit by a trolley or retires or quits?

Having a formal process and documenting it is really improtant.

- Problem mgmt
  - It must first be determined if a problem exists
    - Can the issue be replicated?
    - Is there a known, approved fix?

##### Change mgmt

- Change Mgmt
  - Primary goal is to mitigate organizational risk due to changes to systems and software
- Change process can be initiated through ANYTHING that implements change:
  - Patching to fix bugs and vulns
    - Hotfixes and service packs


- Very important to determine if a patch or service pack (or any change) is even required
  - Also pay close attention to vendors so we are aware of patches/SPs.

- Initiate patch regression testing if possible
  - i.e. test it in a TEST ENV and NOT on production first (if possible)

(Regression testing is testing on non-prod systems and slowly rolling out changes to more prod-like envs as you go.)

It is important that the TEST env REPLICATES the DEV environment EXACTLY.

Also, the testing must occur AFTER the patch/SP is applied.

## Secure Software Design

### Design Processes

If we do our job well making software blueprints, then we'll have a beautiful piece of software at the end.

When we make software with security in mind, the goal is to include features that make our software more secure into the initial design. The end result is a well-designed AND SECURE application.

During the design phase:
- Evaluate software requirements
  - From a security standpoint
  - From a functionality standpoint

- Objective is not to make a product, but a SECURE PRODUCT.

#### Attack Surface Evaluation

Attack Surface is "Any area of your code accessed by unauthorized parties".

This includes any areas that ask for creds by users who will later be authorized (i.e. me logging in to my work computer).

So, if you have webpages that any user can request, and that page asks for credentials to allow only authorized users, those webpages are part of the attack surface as users can still interact with it despite not being authorized yet.

The attack surface is a way of measuring items that MAY be exposed to an attack.

- Measuring attackable components
  - More components = greater risk

Example: Webpage. If it has more text boxes for people to fill in, the risk is higher, because the likelihood of any one of those inputs being vulnerable is higher because there are more inputs.

Complexity generally makes attack surfaces larger.

- Disable functions when not in use!

Reducing attack surface -> Reducing exposed elements -> Reducing risk

##### Attack Surface Measurement

Attack surface of an application depends on understanding the different types of threats and vulns within the app.


To make one:
- Add up all the ways that unauthorized people can access the app
  - The hard part is understanding the different ways...

Remember: If an unauth'd user tries to access the app, and gets prompted to auth, that's part of the attack surface.

Elements associated to an attack surface:
  - Services
  - Guest accounts
  - Weak ACLs

Also, more on building attack surface lists:
- Historical vulns
- Don't just focus on GUIs.
- 'Could this happen to us?'
- All software is different, vulns can vary between products.

##### Attack Surface Minimization

Once we understand the vulns, we can start reducing the attack surface.

- Goal is to reduce attack surface
  - You must reduce exposed items
    - Disable services
    - Lower privileges
  - Anything implemented to reduce the attack surface needs to be well-documented
  - Attack surface needs to be calculated during THE DEVELOPMENT PROCESS as any new code or features can introduce new vulns
  - We may allow an admin to dynamically enable/disable features that increase/decrease attack surface
    - This means they must be well-informed about security implications depending on the features they select
      - i.e. this PRINTINATOR feature opens port 42069 and has some horrifying TELNET printing system that is amazingly insecure, why does this exist oh my god, no sane person will ever use this.
  - Attack surface should be minimized DURING THE DESIGN PHASE so that our software is designed with a SMALL attack surface, and **SECURITY IS NOT AN AFTERTHOUGHT**.
    - This reduces the overall cost as less code is changed in later phases

#### Threat Modeling and Documentation

Threat modeling is about identifying and documenting threats.

The process can be hard as it involves trying to identify many unknowns that may pose a security risk.

We may not be aware a vuln exists, and miss adding it to a list of threats.

All of this info about threats modeled must be communicated to development. This lets them design defenses to mitigate threats.

This documentation must be continuously maintained throughout the product, so it is essentially a living document. We could find a new threat at ANY TIME and need to make sure it's properly documented.

Threat modeling should be started early in the design process. This makes it easy to mitigate any threats with DESIGN CHANGES, not code changes, i.e. waiting until the development phase.

Threat Modeling is everyone's responsibility, and involves many different roles.

Ex: One threat gives us fraudulent data, which makes this example occur at the business-case level.

Ex 2: Another threat occurs at low-level access to software components, which is at the technical level.

Everyone on the team has to be wary of potential threats and assist in the modeling efforts.

##### Threat Model Development

Phases of threat modeling:

1.  Identify security objectives
  - Collect all of the requirements and objectives for our software
    - Legal
    - Security
    - Privacy
    - Data storage
    - *This list can come from many sources and needs to be listed and properly detailed
2.  System Decomposition
  - By breaking our system down, we can understand points of access and how data we must protect is moved through the system
3.  Threat Identification
  - Be thorough and ensure that all possible threats are realized
  - Missing a threat likely introduces a security hole
4.  Mitigation Analysis
  - This step has us review each of our threats and address the risk each poses to our software.
5.  Threat Model Validation
  - We review our list of threats and planned mitigation strategy to ensure...
    - The threat is a valid threat
    - It is mitigated appropriately

##### Documentation

Maintaining good documentation is at the core of doing a good job at threat modeling.

- Provides a roadmap on security related information
- Centralized place for information
- Give info to the development team
  - All team members will be on the same page with respect to the plan


- Objectives
  - Enumerate all threats
  - Record all info about each vulnerability and its risk
  - Record planned mitigation strategy for each vulnerability
  - List any dependencies that different components have on eachother

#### Control Identification and Prioritization

- Security is managed and implemented using Security Controls
  - Primary way of impl'ing security
  - Backbone of mitigating threats
  - Can be thought of as a **Mitigation Package** as controls often resolve more than one security threat

Security controls are safeguards/countermeasures that are used to counteract/minimize security risk. ex:

- Enterprise Auth Mechanism
  - Active Directory
- Security protocols
  - IPsec


- Things to think about with Enterprise Controls
  - ACLs
    - Consider using a model that the enterprise has already implemented rather than making a new model
  - Efficiency and Operability
    - These controls will be used by many users and must work with other systems
  - Must consider the env you work in when deciding what controls to implement

##### Priority

When deciding on what controls to implement, we need to ensure that any decision we make meets our requirements. Ususally there are many solutions that work.

- Look at existing security controls
  - If these meet our requirements, they have priority over adding new security controls.
    - This reduces duplicate work
- Security controls should align controls that are common across the software
  - Access Control Lists
  - User Auth
  - Security Mechanisms
  - *This should be done because it reduces duplicating security controls
    - This reduces the security workload by 'aligning'

##### Design Process

A good approach for designing a secure system is to use the SD3 approach:

- Secure by/in...
  - Design
  - Default
  - Deployment

There is more to consider than just the software. We need to consider all of the aspects of our software that need to be secured so we can leverage the right controls to implement.

- Security provision protocols (low-effort implementation)
  - IPsec
  - HTTPS
  - SSH

Reducing the duplication of security controls is always a good thing. It reduces workload AND reduces risk. If we have 2 different controls that serve the same purpose, and if 1 is compromised, we're at risk. Having 2 systems doubles the risk.

#### Design and Architecture Technical Review

The DaATR is something we must do AFTER the attack surface and threat modeling documentation is finished.

We will use the documentation to review the existing design and architecture to ensure that any changes (like adding sec controls) have been properly integrated into the design.

This improves the current development and ensures we get the best design and architecture to meet functional requirements, and that we meet all security goals.

- Periodic reviews done during development process
  - Security progress review
    - How is the project meeting sec goals
  - Code walkthroughs/Design reviews
    - Revisit these reviews during Code/Design reviews
  - Attack surface examination
  - Documentation on threat modeling

- Review is both technical and process-focused.
  - It's easy to get sidetracked and change the process when making technical changes
  - Stay as true to the planned process as possible
- Security review
  - Done by dev team during dev process
  - Security cannot be added after
    - Much better to build proper security controls into the software to begin with

The review process ensures we pay the proper amount of attention to security.

It also ensures we are progressing enough in the activities that a DaATR covers, like implementing security controls, so we know those activities are being completed at appropriate levels. "Is everything we're concerned about being completed as expected?"

#### Risk Assessment for Code Reuse

When we reuse code, we need to see if it meets our security criteria.

Start by assessing risk by:
- Examine potential security issues
  - Part of this process is reviewing legacy code for vulnerabilities
    - "Legacy code" means any code we are intending to reuse as it must comply with our security requirement

Legacy code will probably have crap documentation, so you'll probably have to do a full risk assessment on it.

If it has good documentation, then you'll probably just have to review it.

- Include the use of legacy code in threat modelling and attack surface minimization efforts
- If legacy code has already gone through this process and was deemed as "SAFE", then it is A MISTAKE to continue to treat it as "SAFE".
  - This is a mistake because there can be a lot of issues/differences when the legacy code is implemented again in a new environment, that can introduce new vulnerabilities.
  - There might also be new vulns discovered that were just never found
- The old code will likely be running in a new environment instead of what it was intended for. This can introduce new vulns or cause problems.

##### Code Reuse

Even though we have to assess legacy code, reusing code still has benefits and is a very common practice.

- Saves time
- Consistency with other apps
- Reduces work
- Software libraries are a great example of code reuse
  - A software lib is a group of legacy code that has a common relationship

### Design Considerations

#### Applicable Methods to Address Core Security Concepts

When developing software, security must not be an afterthought.

We need to consider the following during the development of the software like the methods to address core security concepts.

Secure comms between different components is important.

##### Confidentiality

- Prevents disclosure to unauthorized parties
- Secure secret information
- Identify elements during design phase
- Use encryption

##### Integrity

- Protects data from unauthorized changes
- Can use access control mechanisms
- Can use hashing to verify integrity
  - Also can use digital signatures
- The challenge is deciding how to implement integrity as there are many ways to do it

##### Availability

- About ensuring the system is available
- Common threat is DoS attack
- Other threats exist though, not just DoS attacks.
  - Hardware failures
- Must review threat model -- Not all threats are made by hackers. 
  - Hardware/software failure, etc.
  - System failure
  - Errors
- Backups
- Data replication
- Failover
  - Clustering

##### Authentication

- Can verify someone's identity
- Can be authenticating a person, process, system, or hardware device
- Compromising authentication is extremely valuable to hackers
  - Either getting creds or bypassing the mechanism
- 2FA is very good.
  - 2 ways of verifying the user is the right person
  - i.e. uname+pw and send a nonce to email/cell.
    - They have creds
    - They have access to email/cell

##### Authorization

- We allow an authenticated user to access functionality of a system that they are allowed to access
- Can be  complex to implement
  - Large number of users
  - Large number of securable assets
- Many systems have built-in authorization mechanisms to control access control lists
  - Active Directory
  - As long as these work, use them. Reduces code and uses common mechanisms.
- If built-in mechanisms DON'T meet our requirements, we must make CUSTOM authorization mechanisms to provide the security

##### Accounting

- Often overlooked.
- About measuring activity in the system
  - Usually by logging
- Must decide WHAT to log, we can't log everything.
  - This is done during Design Phase.
- Must protect (encrypt) data in logs.
- Must ensure data in logs cannot be tampered with.

#### Security Design Principles

This section covers the secure design principles that need to be applied in a development project.

- Review information
  - Attack surface analysis
  - Threat model
- Create appropriate controls
  - Ensure CIA

- "Good Enough Security"
  - A line between too little and too much security
    - Too little=insecure
    - Too much=time, money costs, and we hamper processes with unnecessary steps
      - Users do too much work to get thru security
  - Use risk to govern our response

- "Least Priviledge"
  - Users are only granted enough privilege to do their assigned tasks and NOTHING MORE.

- Defense in depth
  - Using multiple, overlapping defenses

- Separation of duties
  - Use multiple people or contrls to complete a task
  - Reduces fraud/error chances and impact

- Failsafe
  - When (NOT IF) a system fails, it does so securely.

- Complete mediation
  - We reverify someone's authentication when they try to complete a task, and verify at every step of the step
    - We can verify if it is STILL the correct user

- Open design
  - We openly review the design of the system
    - This lets us fix faults easier
  - Hiding designs do NOT improve security

- Economy of Mechanism
  - Simpler systems are easier to secure
  - We should strive for a simple design
  - Complexity generally increases error chances

- Least Common Mechanism
  - Reducing the amount of shared components between users or processes
  - The shared component cannot be leveraged to gain access from one user's data to another, or between processes

- Psychological Acceptability
  - Building security controls into a system that are acceptable to users
  - Hard to use controls will be ignored or bypassed by users

- Leverage existing components

- Weakest link

- Single Point of Failure
  - Any single component that brings the system down
  - Redundancy fixes this

#### Interconnectivity

Considerations to take when designing components that manage interconnectivity between software and/or users.

##### Session Mgmt

This is the ongoing comms btwn users and software.

We want a trusted space for comms to occur, and must prevent hijacking

This safe env must be provided PER CONVERSATION so compromized sessions' scope is limited.

- Encrypt comms
  - Web comms usually take place over SSL


##### Exception mgmt

We need to properly handle errors that COULD happen.

- Identify potential failures
- Anticipate failures
- Diagnose and correct failures
  - Failure should be recorded
- IF it canNOT be fixed, it must be logged and NOT send it to the user.
  - Info leak
- This improves usability and security

##### Configuration Mgmt

- Config is stored in files generally
- This manages functional connectivity
- Config:
  - Paths
  - keys
  - Connection strings
  - Init params
- Just imagine if a hacker could modify this data...
- This config data mgmt must be considered during the design phase

#### Interfaces

Interfaces and best practices.

- Generally we think of user interfaces
- But, this can also refer to components

- Remember that some data is handled by components that are external to our system
- We might need to make an interface in order to safely use external components
  - i.e. If we use an OS Auth mechanism, we need an interface to it.
  - We can make one or use an existing interface.

- An interface provides access to data.
  - ACLs
  - Data logs
- Mgmt consoles provide access to this data
- We want to combine techniques to provide access to mgmt interfaces.

##### In-band and Out of band

These are 2 styles of providing an interface.

- In-band
  - Provide access to mgmt data as part of our regular communication channel
  - Easiest to implement
  - Dangerous as mgmt data is on the same channel as normal comms
    - Security issues
    - DoS concern

- Out of band
  - Separate channel for mgmt data
  - You can manage while under attack
  - However, it is more complex to create.
  - This is common to see in many systems

##### Separate Mgmt Iface

- Best presentation (focused) of mgmt data and provides clear presentation of mgmt functions
- Always good to group similar functionality
- Can isolate information better

- Advantages
  - Familiarity
  - Cross-correlation of data to different operational data

### Securing Commonly Used Architecture

Consider infrastructure and env when developing an application.

- Consider type of architecture
- Different architectures we want to leverage

#### Distributed Computing

Client-server and p2p distributed computing models.

- Message queeueueuing

##### Client Server

- Servers
  - Process requests and store data
  - Serve numerous users simultaneously, thousands.
- Clients
  - Single user's work
  - Thin (low local processing) or fat (high local processing)

Client-server can be extended into an n-tier model where there are multiple layers.

- Can be thought of a series of client-server comms in sequence

    Client -> Server 1; Server 1 -> Server 2

Server 1 can be thought of as a client.

A good example of this architecure are web browsers and webservers.


##### P2P

- A set of machines work TOGETHER to process a workload
- Each machine is equal and distributes work equally
- File sharing is a common use of p2p architecture.

##### Message Queuing

- Often overlooked architecture
- Provides a messaging system between two systems
- Kind of like email for applications where 1 application can message another one
- Requires an intermediate server to handle delivery of the messages
  - This server mediates transfer of info
  - Resolves data transfer issues
- Guarantee:
  - Delivery of messages
  - Logging
  - Security

The real advantage of MQ - They allow for one component to be unavailable but still have messages sent to it.

#### Service-oriented Architecture

SOA is a distributed architecture that relies on having a shared service that other components can consume.

SOA can be impl'd with different technologies, but they share a lot of common characteristics.

Characteristics of SOA:

- Discoverable
  - So client components can find them
- Platform Neutral
  - Client components do not need to be running on a specific platform
- Reusability
  - More than 1 client component can use it
- Provides abstracted business functionality
  - Contract-based interfaces
    - Contract details comms standard b/w client and SOA component
- Interoperability
  - Should work with many other types of components

##### Implementation

- Common Object Model (COM)
- Common Object request Broker Architecture (CORBA)
- Web Services (WS) using XML

##### Services

- SOA can provide any service you can think of
- These are the core units of functionality
- 1 service performs a specific action
- Services are Self-contained
- We use SOAP (Simple Object Access Protocol) when using WS
  - SOAP packages components b/w client and service
  - XML does the same job
    - ...b/c SOAP is an XML implementation
- REST (REpresentational State Transfer)
  - Another way to make requests b/w clients and services
  - Data focued

##### Enterprise Service Bus

- Larger scale version of SOA
- Intended to be a single place where all the comms b/w data producers and consumers are handled
  - By doing this we can monitor and control routing of messages and send data to multiple clients simultaneously
- Convert protocols
- Handle defined events
- Typically supports many protocols
  - Message queueing
  - XML
  - EDI
  - WSDL
  - REST
  - DCOM
  - CORBA


##### Web Services

- Allow communication b/w elements (like client/service)
- XML is used for comms
- WSDL (web services desc lang) (wizz-dil) is a description of comms standards that the web service supports
- WS can be impl'd with either SOAP or REST.
  - REST is more data focused
  - SOAP only uses XML but has security advantages

#### Rich Internet Applications

- RIAs are apps that are being trasnferred and communicated via the web b/w web server and web client
- Client does more processing than just showing a webpage
  - How 'rich' the app is depends on how much processing the client does
- Benefit of this is that client doing processes reduces communication b/w client and server
  - More speed on client
- Frameworks
  - Flash
  - Java
  - Silverlight

Because we're running code within a framework, security is important.

##### Client-side Exploits or Threats

Don't assume that clients will use validation code given to them!

Don't rely on client-side validation! The client can send WHATEVER byte stream (and yes, EVERYTHING is a stream of bytes) to your server/component that they feel like!!!

- Hackers can corrupt/compromise data given to them/sent to the server.

- Serverside validation is a must.

- Client cannot be trusted
- Client can be running malware or misconfigured

##### Remote Code Execution

RCE is when 1 machine causes another to execute code.

- Common vuln in RIAs
- Very serious threat
- Can cause user to gain control of server
- Code and data must remain SEPARATE

#### Pervasive and Ubiquitous Computing

- Devices are powerful and varied
- People run apps on anything, anywhere, in any format.
- Phones, tablets, watches

##### Wireless

Many of these devices rely on wireless networks. No cables.

- 802.11/802.15
- BT
- WiMax
- LTE

These networks can be unsecure and must not be trusted.

##### Location-based

Location services can be used to 'know' where a device is geographically.

- Marketing use

You can restrict functionality depending on locale, although users can forge location info.

- A person's location can be considered sensitive, and may need to be treated as PII.

##### RFI

Non-contact comms method that requires a transmitter and receiver.

Transmitter broadcasts short range freq, and ANY receivers close enoguh can read it.

RFI tags in products in stores are an example.

RF device frequencies are regulated by countries.

###### NFC

Near-field comms.

- Protocol that uses RFI over short distances (few inches).
- Payment: 'tap to pay'
- Easy to compromise: Read over long distances and also needs no authentication.
  - Can steal credit cards
  - Can siphon data from CC

##### Sensor networks

- Autonomous sensors that measure things
- i.e. automatic weather stations
  - Rainfall
  - Weather
  - Communication efficiency
- Each station is a 'node'
- The node can be physically compromised as they are usually in remote locations

#### Integrating with Existing Architectures

- Dealing with the interaction between the software we're making and the components from the existing architecture
- We must evaluate if all components (including existing ones) meet requirements of our app not only from functionality but also a security standpoint

- Cross-integration
  - This means the integration goes both ways
  - Our app can also provide functionality back to the enterprise, like data
  - Benefit: Enterprise architecture has more functionality.
    - But adds work. Is it worth it for the future?

- IwEA considerations
  - Cost
    - Is the time+money+effort worth the functionality later?
    - Usually short-term and long-term payoff

#### Cloud Architectures (Deployment Models)

"Cloud" is a phrase used to describe an architecture that is scalable and automatically provisioned based on demand.

- Private cloud
  - An organization provides a cloud env internally
  - Infra cost can be high
- Public cloud
  - 3rd party companies provide cloud architectures to more than 1 customer
  - Customers are isolated from oneanother
- Community cloud
  - Cloud services focused on supporting a group of people
  - Services may be specialized and focused on 1 group of people
- Hybrid cloud
  - Coupling internal and external services
  - Does not necessarily mean 2 cloud structures that are both internal+external
  - More commonly, internal non-cloud services work with external cloud-based services.

##### Software as a Service (SaaS) 

- Software runs in the cloud
  - FB, Gmail
- Client-browser based
- Benefits
  - No contact distribution (no deploying to machines)
  - Instant updates
  - Software is restricted by virtue of running in a browser
  - '0 footprint' on client
- Data security
  - Data is stored internally to the SaaS, in the cloud

##### Platform as a Service (PaaS)

- Step above SaaS
- Offers a complete platform -- not just 1 piece of software like SaaS
- PaaS requires client subscription for the services
- Client does not manage infrastructure
  - Network
  - Servers
  - OS
  - Storage
- Ex:
  - Email
  - Web services
  - Data services
- Can offer significant savings because of the economy of scale

##### Infrastructure as a Service (IaaS)

- Completerer platform
- Clients can manage infrastructure
  - Networking
  - Provision storage
  - Deploy software
- IaaS fills the gap between SaaS to PaaS
  - Gives more control over service config
  - answers "What if PaaS doesn't offer the platform service we need"
- Downside is extra config and managing the environment

#### Mobile Applications

Mobile apps are very common and new.

- Phones
- Tablets
- Anything handheld
- Comes with mobile OS

##### Mobile apps

- Designed for mobile devices
  - Limited processing
  - Limited mem and I/O

- Your app might thrash or freeze
  - Encryption may do this

- App stores offer authorized apps
  - Apps can be private
  - Commercial apps can be downloaded by anyone

- Every mobile vendor has a different platform
  - What does this mean for security?
    - Different OSes? Disabled features? (i.e. no ios secure enclave)

### Technologies

One challenge today in developing software is how many different technologies your solution will be based on.

Understanding as much as you can about technologies in and around the solution is crucial.

#### Authentication and Identity Management

Steps:
- Identify user
  - i.e. uname
- Authenticate user
  - i.e. password with username upon login
- Authorize user
  - Based on user identity, determine

Authentication is challenging because different verification methods can be circumvented: Passwords can be stolen or guessed!

Identifiers:
- Password
- Digital Certificate
- Biometrics

##### Authentication Factors

- Something you know
  - Password
  - PIN
- Something you have
  - Token
  - CC
- Something you are
  - Biometrics
  - Voice
  - Retina

Consider:

- How easy is it for one of these factors to be stolen/impersonated?
  - Steal a phone
  - Steal a credit card #
  - Can't steal an eyeball easily...

##### 2FA

Two-factor auth.

Using two auth factors increases confidence in the authenticity of the entity attempting to authenticate. This is because it is less likely that both factors are compromised, when compared to 1 factor.

Pros:
- Reduce theft and fraud
- Greater assurance of valid authentication
Cons:
- Can be compromised by malware (record creds or requests)
- Can be compromised by MiTM or fake login systems

##### Federated Identity Mgmt

- Directory Services
  - Central storage for credentials and users
  - Active Directory aka LDAP
    - There are likely multiple sources of info about users
    - It can be hard to sync this info with the central repository
  - Some systems may need to keep local DB copies
  - Problems may occur with stateful systems
  - May be a lack of consistent policies

To help manage these issues, User Provisioning is likely to be automated.

- User Provisioning
  - May be slow
  - May be inconsistent

#### Credential Management

- Creds are used for auth
- Cred Mgmt refers to how creds are protected

Can have:
- Passwords (weak)
- Tokens
- Biometrics
- Certificates (strong)

##### X.509

- Int'l standard for PKI (public key infrastructure) and PMI (Privilege Mgmt Infrastructure)

- Standards for certs and cert handling

- PKI is asymm encryption

- Cert fields:
  - Version and serial #
  - Signature algo
  - Issuer and validity
  - Subject
  - Public key
  - Usage of cert
  - Any other extensions

X.509 is a standard, introduced by the International Telecommunication Union in the late 80s.

The Internet Engineering Task Force (IETF) established the working group for X.509 standards and established the RFC 3280 standards, and the RFC 5280 standards (v3 of certs).

These standards established the PKI by providing a framework for cryptographically signed certs.

- Prevents altering of cert.
- Provides structure to verify the source (or all parent certs and other sources) when you have a cert chain.

Certs are based on cert authorities.

##### Single Sign-On (SSO)

- Certs and PKI are very involved in SSO

- SSO lets users sign in once and REUSE credentials.
  - From user's perspective, they are only prompted to authenticate once. Great for them!
- This is accomplished by STORING THE CREDENTIALS outside the app, generally in a directory service.

- Kerberos
  - Auth protocol that uses PKI to establish auth
  - Allows SSO using 'tickets'
    - tickets can verify user identity to use services
- Security Assertion Markup Language (SAML)
  - Claims-based auth mech
  - Used to implement SSO
  - Used for the web
  - Allows a user to create a SAML-based claim that is digitally signed by the author
  - Claims are similar to passports and can be used to authenticate into different systems without needing to reauth.
- OpenID protocol
  - Similar to SAML
  - Allows for authentication to happen over the web
  - THE most popular authentication mech on the web, used by many 'big names'
  - Allows a user to use a single cred to auth for multiple resources

#### Flow Control

Controlling flow of info from 1 system to another is important when securing systems.

##### Firewalls

- Policy enforcement device that controls the flow of info b/w 2 networks by allowing or blocking
- Can impl many rules
- Fundamental
- "Next Generation" firewalls
  - Allow for more control over network traffic
- Generally just do STATELESS packet analysis and inspection (wireshark with a gun)
- Next gen firewalls are STATEFUL:
  - "Did this comm b/w Alice and Bob __START__ from _inside_ the network? IF SO, let the incoming traffic in.
  - This "thorough (stateful!) packet inspection makes 'next gen' firewalls very useful and 'smart' tool.
- Firewalls can act with Intrusion Detection Systems (IDS) that monitor intrusion patterns (i.e. repeated pinging, login, etc.)
  - When necessary IDS can tell firewall to block that intrusion attempt dynamically

##### Proxies

- Control (redirect or modify) traffic flow
- Middleman that protects internal device
- `squishy_mysql_db <=====> proxy <=====> internet <====> evil_spikey_hacker`
- `employee_pc <=====> proxy <=====> internet <====> hxxps://coolguns.ru/`
- Internal devices are  not DIRECTLY contacted by external systems
- Proxies also use a set of rules, just like firewalls
  - Can be simple or complex
- Benefit: Caching. 2 users go to a website. Proxy caches response (images! JS!) and doesn't need to ask coolguns.ru for double the pictures.

##### Application Firewalls

- Type of firewall/gateway that governs ONE application.
- Allows controlled access to just that application
- Can monitor traffic in either direction
- Block traffic
  - Based on rules
  - Or to stop misbehaving applications

##### Queueing technology

- Like email for systems

- Message transport
  - Synchronously
  - Asynchronously (preferred)
    - Alleviate congestion
    - Stop traffic loss

- Reduces congestion by reducing send/receive messages b/w systems
  - When network is congested, W/O queuing, it only gets worse: Systems just retry sending messages.
  - With queueing, it doesn't matter how backed up MQ is: Systems send it and continue their tasks.
  
- Vendors
  - Microsoft Message Queues
  - IBM MQ Series
  - Oracle

#### Logging

- Record actions taken by a user in a system
- Protects against repudiation

- Record...
  - What action was taken
  - By who
  - Where was the action taken (What system)

- Logging requirements
  - HIPAA
  - SOX
  - PCI
  - DSS
  - EOC


##### Logging criteria

What information do we record?
- Cause of the issue
  - Events before issues
  - Challenge is to balance 'too much' and 'not enough'.
- What impact does logging have on resources?
  - Processing
  - Storage (more important)
- Local logging
  - Simplest solution
  - MUST be secure and integrated later into an enterprise solution
- Third party logging tools
  - Security information and event mgmt solution (SIEM) can do this

##### Syslog

The "System Log" for a system.

- Records system actions
  - User logs on to OS
  - Built for UNIX and used in both UNIX and Linux

- Approved by the IETF
- Uses UDP and TLS to communicate log entries to other servers, like a central server.
- Microsoft has no equivalent (take that!)

#### Data Loss Prevention

Data is critically valuable.

- DLP
  - Reduce risk
  - Protect data
  - This is our last line of defense

DLP is not just about maintaining good backups.

Some software uses 'profiles' to detect data access and potential loss
- Size of transfer
- Destination
- Data type

##### Reduce Risk

Protect data...
- in motion
  - Encrypt data when moving it
  - Protect at the packet level
  - Limit the types of channels data is transmitted over
    - Block certain email, encrypt others
- at rest
  - Protect network shares
  - Protect databases
- in use
  - Network endpoint monitoring
  - Device (i.e. usb key) monitoring
  - Monitor where data exits (preventing usb key copying)

#### Virtualization

- Layer b/w OS and hardware
- Separates real hardware and OS inside virtual OS
- Can run multiple OSes
- Allows for flexibility in upgrades and scaling
- Security advantages
  - Can contain malware
  - VMs are logically isolated from hardware and other software

- Vendors
  - VMWare
  - Microsoft
  - Xen

##### Benefits

- Cost savings
  - Less hardware
- Efficiency
  - (-) add overhead
  - Less administration in many aspects
- Isolation
  - Applications
  - Data
  - Platforms
  - Storage
  - Memory
- Scalability
  - Cloud computing
  - Provisioning VMs

#### Digital Rights Management (DRM)

- Content can be COPIED even within our own environment
- "Data wants to be free"
- Restricting content outside our env
- Protect intellectual property
- Copy protection
  - Limit usage rights
  - Guarantee authenticity
  - Guarantee integrity

##### Rights Expression Language (REL) 

Defines license and perms/restrictions on content.

- Machine readable
- Many formats, XML, JSON, etc
- also...
  - ccREL
  - ODRL
  - MPEG-21
  - XrML

#### Trusted Computing

##### Trusted Computing Base (TCB)

TCB is all of the hardware, software, and firmware components that are CRITICAL to securing a system. If a component's compromise would compromise the entire system, it is part of the TCB.

TCB is really about privilege escalation.

##### Trusted Platform Module (TPM)

- Hardware solution for crypto
- Usually a chip on the mobo

- Purpose is to be a totally isolated device that handles crypto instead of OS doing it in software
  - This isolation secures the crypto
  - Can handle encryption keys in an isolated environment

- Controversial: TPM chips can be used to LIMIT the software that runs on a device

- Features:
  - Hash gen
  - RSA gen
  - Enc/decryption

##### Malware

- Malicious software.
  - Viruses
  - Worms
  - Trojans

- Very complex threat

##### Code Signing

- One of the best ways to protect code

- Using digital signatures to sign code
  - Executables
  - Scripts
- Provides info
  - Author of software
  - Integrity level
  - The expected hash of the code

#### Database Security

Databases are the primary locations for data in most enterprises.

##### Encryption

- Protects data at rest
- Encryption Mgmt
  - Database Management System itself provides encryption
  - External crypto resources
- The keys are the most valuable asset

Consider:
- Level of risk of the data
  - What if it gets compromised?
- Usage pattern
  - How is it transmitted?
- Sensitivity of the data
- Encryption handling
- Encryption options

##### Triggers

Allows a database to react to a specific activity.

- i.e. 'add 1 to this column when you see "potato" in this row'
- Can record useful info
  - Changes
  - Additions
  - Execution of routines or commands
- Can prevent a command from executing based on logic
- Can be used for custom DB logging

##### Views

Another way to help protect data in a database.

- Shows dynamic 'views' of data in one or more tables
- Views are LIKE tables, but don't contain any original data, just data extracted from tables.
- Tables
  - Contain all the data
  - Records
    - Addresses
    - CC#
    - SSN
    - Names
- Views can be used to restrict what data is available (i.e. remove CC# and SSN) without providing the entire record in the table
- Can be used to combine data from 2 tables

##### Privilege Mgmt

- Main way that security in DBs is configured
- Internal access control
  - Similar to ACL-based controls
- Can be integrated with system security

#### Programming Language Environment

The env in which a dev creates and edits the code.

- Source code turned into binary/machine code
  - Compilers
  - Interpreters
  - Both

##### Common Language Runtime (CLR)

- CLR used in MS .NET langs
- Provides an env for 'managed code' to run in
  - Managed code: code intended to run in the CLR

1.  Dev makes managed code
2.  Compiled into a Common Intermediary Language (CIL) (called MSIL)
3.  MSIL compiled into machine code, using JIT compiling managed by the CLR
4.  CLR executes MSIL code

CLR provides:

- Traps
- Index checking
- Sandboxing
- Gargabe collection

##### Java Virtual Machine (JVM)

- Similar to CLR
- Java compiles to java bytecode (not machine code)
- Java bytecode must be executed in the JRE
- JRE JIT compiles bytecode to machine code

##### Compiler Switches

- They control program construction
  - Memory
  - Exception handling
  - Stack protection
- /GS flag tries to prevent buffer overflows
- /SAFESEH flag restricts the program to only call valid exceptions
- ...more

##### Sandboxing 

A sandbox isolates the executing code away from system resources or other code.

- Useful for executing untrusted code
- Mediates access to...
  - Memory
  - Network
  - Filesystem

#### Operating Systems

OS is software that manages the hardware and controls the environment that software is executed in.

- MS Windows
- UNIX
- Linux

- Mainframes
- Desktops
- Laptops
- Mobile devices

- OS provides a functional interface to the hardware.

- Different types
  - Multi user
    - Must isolate interuser processes and data
    - Needs more resources
    - Restrict access to resources on a per-user basis
  - Realtime (RTOS)
    - Specialized OS
    - Processing of data is most important
      - Must occur as efficiently as possible

#### Embedded Systems

Dedicated systems designed to perform a specific task.

- OS must be streamlined as devices are generally small
- Time-sensitive constraints
  - i.e. Streaming video to security feed
- Want to use minimum amount of hardware generally

ex:
- Vehicle nav
- Factories
- Music players

##### Control Systems

Specialized systems that allow automated control over hardware.

- Paired with motors/actuators
- May use a Programmable Logic Controller (PLC)
- Or a Remote Terminal Unit (RTU)
  - That sends data to a Supervisory Control and Data Acquisition System (SCADA)

CSs are used in oil and gas monitoring or environment monitoring systems.

##### Firmware

Firmware is a way to program a device.

- Software code is deployed via physical wire onto device memory, or make it impossible to alter
- Used when we do NOT want the software to change

- Stored on...
  - NVRAM
  - ROM
  - EPROM
  - Flash

- BIOS chip is an example of firmware.
  - Can be updated but not commonly.

## Secure Software Implementation and Coding

### Declarative vs Programmatic Security

Testing, analysis, and reviews are important when coding.

#### Declarative Security

- Security can be represented in two ways in code
  - Declarative (part of app/container)
  - Imperative (in code)

- DS defines security relations with respect to the container
  - Security rules are stored in a config file
  - These files store authentication and authorization settings for an app/container

Example: ASP.NET Web.config file with authentication and authorization sections. ONE file has those security config settings.

A benefit of this is that DS is flexible. It's easy to modify rules after code is compiled.

- Security rules are defined as part of DEPLOYMENT, NOT AS PART OF THE CODE.

- In DS, security is managed by the operational personnel, NOT the dev team.

#### Programmatic Security

AKA Imperative security.

- Security is written into the application code (aka container).
- NOT stored in a config file.
- Less flexible.
- Offers very granular security (fine control)
- Enables enforcement of very complex rules
  - You cannot do this with declarative security.
- Can make code less reusable


- Design considerations:
  - Decide whether to use Declarative, programmatic, or a MIX.
  - Make this choice early in the design process as it is a foundation
- Then, start building the app
- The job role with PS is of the developer and not deployment team.

### Vulnerability Databases and Lists

#### OWASP (Open Web Application Security Project) Top 10

- Open source community intended to improve web app security
- <https://owasp.org/>
- OWASP Top Ten is a list of common security flaws
  - Come with a description of the risk
  - Examples,
  - And how to avoid that risk.

1.  Injection is the #1!
  - SQLi, LDAP
2.  Broken auth and session mgmt
  - Allowing attackers to compromise passwords, keys, security tokens
3.  XSS
4.  Insecure Direct Object References
  - Exposing internal object references to attackers
    - i.e. "give me a filepath, my webserver will just give it to you"
    - Allowing attackers to provide primary keys directly
5.  Security Misconfiguration
  - Default security settings
6.  Sensitive Data Exposure
  - Leaving IDs, passwords, SSNs exposed
7.  Missing Function Level Access Control
  - Not checking if users are authorized to execute functions
8.  CSRF
  - Causing victim's browser to forge HTTP requests
9.  Using Components with Known Vulnerabilities
10. Unvalidated Redirects and Forwards

#### Common Weakness Enumeration (CWE)

- Dictionary of common software weaknesses
- Meant to help devs and orgs ensure their apps aren't vulnerable due to software weaknesses
- <https://cwe.mitre.org/data/published/cwe_v2.8.pdf> (old btw, 2014-07-31)
- <https://cwe.mitre.org/data/published/cwe_v4.2.pdf> (2020-08-20)

- Breaks down common weaknesses by type and environment:

```
CWE-5: J2EE Misconfiguration: Data Transmission Without Encryption
CWE-6: J2EE Misconfiguration: Insufficient Session-ID Length
CWE-7: J2EE Misconfiguration: Missing Custom Error Page
CWE-8: J2EE Misconfiguration: Entity Bean Declared Remote
CWE-9: J2EE Misconfiguration: Weak Access Permissions for EJB Methods
CWE-11: ASP.NET Misconfiguration: Creating Debug Binary
CWE-12: ASP.NET Misconfiguration: Missing Custom Error Page
CWE-13: ASP.NET Misconfiguration: Password in Configuration File
CWE-14: Compiler Removal of Code to Clear Buffers
CWE-15: External Control of System or Configuration Setting
CWE-20: Improper Input Validation
CWE-22: Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')
CWE-23: Relative Path Traversal
...
```

- Give you an example of the weakness
- Give you a mitigation for the weakness
- Give you references about how to fix or exploit the weakness
- Very cool document. Useful.

### Defense Coding Practices and Controls

Ensure software will continue to operate despite changes or attacks.

#### Concurrency

Already know this. Readers, use Google.

#### Configuration

- When installing an app to a system (i.e. OS), the system itself could be misconfigured.
- Other apps can also be vectors for exploits
- Good example is web apps.
  - Wapps are installed on web servers.
  - Wapps access data on a DB server.
  - Any of these systems and/or OTHER software installed on them are vectors themselves.
- OS PLUS software can be vulnerable to attack.

##### Preventing configuration attacks

- Establish a process to harden the OS
  - Remove extra software, patch the system, add firewalls
- Modify default configs of an app AND system
  - Default accounts/shitty passwords
  - Authentication methods
  - Folder locations
  - Review all default settings
  - Use config files for your application settings (conn strs and enc keys)

#### Cryptology

The use of crypto fns within your app.
- Hashing
- Encryption

- Vulns in crypto logic are pretty serious.
  - You should make careful decisions when impl'ing crypto.

- Some flaws:
  - Not encrypting sensitive data (i.e. config files or a db)
  - Not securing crypto keys
  - Using old crypto APIs

##### Mitigation

- Encrypt any sensitive data at rest
- Don't allow sensitive data to cross trust boundaries
  - Military segregates secret vs classified vs unclassified networks
- ***Don't roll your own shitty crypto!*** It will suck! It won't last long! Math nerds ARE SMARTER THAN YOU on the topic of crypto!
- Don't use old APIs
- Ensure software is ready to accept new hashing/crypto algos

#### Output Sanitization

Sanitization: Converting info from a harmful form to a non-harmful form
- ...whatever 'harmful' means
- *In my intepretation: Don't mix code and data. Encoding IS THE BEST FORM OF sanitization, if it is possible to encode in the current context

- Two types:
  - Input - Performed on data before it is brought into a system
  - Output - Performed on data before it is presented into the user
    - Generally by encoding, ex. `<` as an HTML entity becomes `&lt;`

- Ensure that the integrity of the data is maintained

##### Methods

- Stripping (blacklist... :( )
- Substitution (blacklist... :( )
- Literalization (probably encoding? Never heard of this.)

** NOTE The training I am taking is WRONG. This is bad advice. Don't use blacklists. Encode or use whitelists. **

#### Error Handling

- Validate input!
- Perform output error handling.
  - Make sure that error messages do not divulge sensitive information.
    - "Keep SQLi blind" so to speak.

##### Best Practices

- Use non-verbose messages for users
  - No stack traces.
  - Don't say "uname wrong" or "pw wrong", say "invalid creds" or "login failed".
- Always have the software fail to a secure state
  - ex: User gives wrong uname+pw, we can tolerate a minimum amount of attempts, and then lock the account.
- Limit the number of user errors before an action is taken and a security event is recorded
  - Fail to login 3 times
- "Clipping level" is the number of errors before action is taken.

#### Input Validation

- All data input should be considered malicious
- All data input must be validated before being processed
  - Check the type
  - Check the range (number or date)
  - Check for illegal characters
  - Verify minimum and maximum lengths of data


- Can use regex
- Can use a whitelist or (NOT!!!) a blacklist

- Can validate at either client or server, or both (*NOTE: This is bull. Don't rely on client-side validation! Clients can lie!)

#### Logging and Auditing

It is important to be able to log and audit actions within the application.
- "Who performed this action at this time on this system?"

Forms of auditing (order of importance):
- Error logging - to a file
- User action logging - to a file or db
- Administration logging - to a file or db
- Database logging - feature of the db

* Create an area in the app that allows for logging and auditing to be configured

#### Session Mgmt

Users should authenticate to the app before they gain access.

Once the user is authenticated, they are given a session ID.

The session ID is essentially a token that gives users access, so obtaining a session ID gives access.

- MiTM attack

Once the hacker impersonates the user by stealing the session ID, how does the app know what requests are actually coming from the user?

- A security token (CSRF token, anti-forgery token, nonce, etc.) can be used with the session ID to ensure that a stolen session ID is useless.

#### Exception Mgmt

- I know what exceptions are
- Basically, unexpected actions/state within an application occur/take place.


- Use try-catch
- Filter exceptions
- Exceptions contain sensitive info, useful to attackers.
- Be consistent with exception handling
- Keep error messages vague

#### Safe APIs

- API example: WMI or Crypto lib.
- NOT just web apis. Can be any application programming interface.

- Old APIs may be insecure.
  - Org must assess how old they are and update if need be
  - ex: Old SSL APIs are full of holes. Heartbleed bug example. Also, old crypto APIs suck bigly. They may be DES or 3DES, etc.

- API security considerations:
  - Don't use old or banned APIs
  - Replace old APIs with similar, secure APIs
  - If your company is creating an API as an interface to your app logic:
    - Authenticate the request
    - Audit access to the system and the API calls
    - Encrypt sensitive data

#### Type Safety

TS is a feature of langs that prevents data type errors (aka casting errors)

Ex: Storing float in an int, or multiplying a string by an int.

- Methods of impl'ing TS:
  - Static: Types are assigned at compiletime. Compiler catches type error during compilation. Java and C do this.
  - Dynamic: Types are assigned at runtime. Compiler does not catch errors at compiletime.

#### Memory Mgmt

MM is the concept of bieng able to manage memory resources and freeing memory when not needed. It is generally complex.

MM and allocation are shared responsibilities b/w OS and apps.

- Managed code: Excels at memory mgmt (C#, Python, Java)
  - Sandboxed. Runtime engine controls memory.

- Unmanaged code: Wieldy and complex (C, C++)
  - Devs control GC, thread pooling, buffer overflows, etc.

##### Type Safety

- Linked to memory safety
  - An application can ONLY access memory that has been allocated to it
- Prevents errors in programs with different datatypes
- Type-safe code always stays within allocated memory range

##### Locality

- Defines predicatability of memory references
  - Future memory accesses are close to previous references
  - NOP sled
- Memory attacks leverage locality
  - Buffer overflows
- Address Space Layout Randomization (ASLR) helps defend against locality attacks

#### Configuration Parameter Mgmt

Settings stored in a config file in the app, designed to be alterable.
- Startup vars, conn strings, cryptographic info.

- Startup vars
  - Malware can attack a bootstrap process (phase that inits env vars)
  - Startup vars must be protected from change outside the app
  - Config files must be secured

- Encrypt sensitive data using a secure algo and large enough block size.

- Cryptographic agility
  - Don't hard-code the cipher
  - Allow for algos to be changed

#### Tokenizing

- Some industries prevent storage of sensitive info after a txn (like cc#)
  - Tokenization is the act of replacing sensitive data with unique symbols in order to retain the information


EX: Instead of storing a CC#,
- Generate a random value (token) based off of the txn info and last 4 digits of CC#.
- Because the token is not based on the actual CC#, it cannot be used elsewhere.

#### Sandboxing

Sandboxing is executing code in isolation from the host OS.

- Used to let untrusted/unverified code run
- Can allow for various system interactions across:
  - Memory
  - Network
  - Programs
  - Devices
  - Filesystems

### Secure Coding Practices

#### Source code and Versioning

`V a.b.ccc.ddd`

- a = Major Version Number
  - Major change in functionality
- b = Minor Version Number
  - Minor features or fixes
- c = Build Version Number
  - Current build or compilation of application
- d = Revision Number

Versioning ensures devs can label versions, and roll back code. Also lets you track changes to code.

Also prevents "Regenerative Bugs" -- Fixed bugs that occur because the wrong version of the code is being used.

Versioning impl'd by managing changes with version control and VC numbers.
- VC# uniquely identifies each release

VC is extremely important for large apps.

Security concern: VC software should have a 'checkout' feature.
- Stop other devs from changing code I am trying to modify

- Automated systems are good
  - VSS, CVS, TFS
  - Mitigates human error
  - Increase efficiency

#### Build Environment

Best practices.

- Software build is a multistep process.
  - Compilation - Compile source code.
  - Linking - Refs to deps
  - Testing
  - Sign/package
  - Distributed - Install app.

Build Env integrity MUST be ensured.

- System in secure state, ACLs, VC software to ensure correct version of software is built


- Various tools exist for compile-link-test-sign/package-distribute.
  - Tools must be configured correctly to mitigate issues with problematic code

- Automated tools in IDE help eliminate errors
  - Static and dynamic checks

#### Peer-based Code Review

Dev team sits down an reviews each other's code.
- Fresh eyes offer valuable insight
- Can take time and analysis effort.
  - Maybe use automated tool. HOWEVER. Not a replacement for sit-downs w/ buddies.

Purpose of PbCR:
- Locate code weaknesses
- Locate code vulns
- Assist devs with development
  - Create succinct code
  - Share 'best way' to do XYZ
  - Clean up code

Procedure of PbCR:
- Code author explains code line-by-line
- Team members draw from their experience to identify potential issues
- Goal is to identify inefficient or insecure code

Result:
- Clean and secure code
- Wider group understanding of code
- Prevents unwanted code from being included
  - Easter eggs, dev crap they left in, logic bombs, backdoors
- Educates junior team members

PbCR can uncover:
- Inefficient code
- SANS/OWASP lists of errors
- Exception handling
- Injection flaws
- Crypto calls
- Deprecated functions
- Privilege levels
- Logging
- Poor key/pwd mgmt practices

#### Code Analysis

Static vs dynamic code analysis.

Code analysis is the inspection of code to identify vulns.
- Static,
- Or dynamic.

Static: Inpsect code w/o executing.
Dynamic: Inspect code WITH execution.

##### Static Analysis

- Performed on source OR compiled code
- Performed by humans or tools
  - Humans perform code review, limited to high-level langs
  - Tools can be used for any codebase
- Usually performed by source code analyzers and binary analyzers
  - Typically used to identify vulns by comparing against common vuln patterns
    - Abstract syntax graph
    - Taint analysis
  - Identify issues that human analysis can miss


##### Dynamic Analysis
- Performed on target or sim system
- Important for embedded systems
- Requires specialized automation for specific testing
- Commonly performed with tools
  - Dynamic test suites
    - Confirm:
      - Parallel functions
      - Thread mgmt
    - Detect:
      - Race conditions
      - Memory errors

#### Anti-tampering Techniques

Prevent unauth'd modification of code/data.

- Obfuscation: Obscure source code W/O altering function.
- Anti-reverse-engineering: Remove symbolic info from software.
  - Class name
  - Member name
  - Global vars
- Code signing: Generate hash of code and encrypt with private key.

- Code signing ensures authenticity
  - Uses PKI
  - Does NOT ensure code is error free or functions
  - Should be used for ALL software distr.
    - Esp over internet

How to sign code:
- Code author makes digest of code (hash)
- Signer's priv key used to encrypt digest
- Code and signed digest is sent to User
- User hashes code using same hash fn as author
- User decrypts signed digest with their pub key and compares their computed hash with the decrypted hash
- If they match, code integrity is verified.

## Secure Software Testing

### Testing for Security and Quality Assurance

#### Testing Artifacts

Testing should occur throughout the dev process by software testers, also known as the 'QA' team.

- Artifacts are created during the dev process:
  - Use case
  - Class diagram
  - UML diagram
- Software testing requires its own artifacts:
  - Test plan
    - 1. Test requirements
    - 2. Methods performed during testing
    - 3. Test coverage
    - Huge artifact. Identifies objectives of the test itself.
  - Traceability matrix
    - Responsible for correlating the design documents to the test documents
  - Test case
    - Responsible for testing reqs from test plan, and converting those reqs into individual test cases to actually test the app
  - Test script
    - Procedures that replicate how a user interacts with an application.
  - Test suite
    - Group of test cases that are logically grouped together
  - Test data
    - Set of values used to test the functionality of the application.
  - Test harness
    - The global term for the software, data, and the config

- Testing group should work CLOSELY with development group, to ensure comprehensive testing coverage.
- Create and monitor tests for each level of integration
  - This ensures relevant properties are monitored at appropriate intervals

#### Functional Testing

- Performed to ensure software functionality aligns with end user expectations
- Determines reqs compliance with:
  - Reliability: Functions as expected at all times
  - Logic: Step-by-step debugging
  - Integration: Test all components together
  - Regression: Verify code changes do not break other code
  - Resiliency: Software performance under attack


Typical functional testing steps:
1.  Identify the functions software is supposed to perform
  - Look at the requirements document
2.  Create "input tests" to test data that will be given to the program
3.  Determine the expected output test results
4.  Execute test cases corresponding to the functional requirements
5.  Determine functional compliance
  - Compare the actual and expected outputs

#### Nonfunctional Testing

NT will test aspects of the application that are not direct functions that it performs.

Not "I can enter a user's phone number".

Like:
- Reliability
- Availability (app is available at all times)
- Performance
- Scalability
- Security

#### Security Testing

- White-box
  - Tester has full system knowledge 
    - Working components
    - Complete documentation
    - Source code
    - Configuration parameters
  - Generally done early in dev cycle
  - Advantage is that tester has full knowledge of system


- Grey-box
  - Partial combination of B+W box testing
  - e.g. "Here's some environment info. No source code though."
  - Rare outside of internal testing env


- Black-box
  - Tester has no knowledge of software being tested (just like an outside attacker)
  - Common in advanced system testing
    - Pentesting
  - Focuses on behavioral characteristics of the app itself

#### Environment Testing

- Apps operate in specific environments
  - Environments also need to be tested
- Test data movement across environmental boundaries
  - End to end of the application

- Across trust boundaries and b/w modules, be aware of:
  - Security credentials
  - Permissions
  - Access Tokens

#### Bug Tracking

Bugs are a part of dev't.

Can come from:
- Design issues
- Coding
- Deployment issues
  - i.e. config

Issues are managed via remed with 3 states:
- Fix defect (mitigate risk)
- Defer functionality of software to later version (transfer risk)
- Replace software (avoid risk)

Defect removal is NOT ALWAYS POSSIBLE.
- App functionality may be lost
- May be too costly (time and/or money) to fix

Defects can be tracked systematically to capture:
- Where it occurred
- The part of code it happened in
- Build number
- Who dev'd or discovered it
- If it is exploitable

Bugs/Errors:
- Errors can be located through many methods
  - Manual reviews
  - Automatic tools
  - Automatic pentesting
- Errors can range in severity and impact
- All errors should be tracked regardless of perceived risk
- Error collection and feed back to devs can:
  - Reduce errors over time
  - Teach devs

Vulnerabilities:
- Can be exploited for malicious intent
- Range in severity and can be quantified by impact on system
  - Can use LOW/MED/HIGH
  - Rating it is important
- Typically identified thru pentesting
- Errors that result in vulns are often very damaging

#### Attack Surface Validation

ASV identifies application risks that exist in software.

- This attack surface must be documented throughout the dev process
- ASV identifies:
  - Parts of app/system that need to be tested for vulns
  - Identify high risk code
  - Identify changes so threat assessment can be performed on changed code
- Testing elements and updating the attack surface will:
  - Provide feedback to dev team
  - Ensure attack surface objectives are met
- To ensure attack surface objectives are met, test:
  - Level of code untrusted users can access
  - Quantity of elevated privilege code

#### Standards

Standards to impl't proper sec testing.

- Int'l standards are critical for proper QA for app design and appsec.
- Quality is suitable for use as defined by specific requirements (defined inside standards themselves)
- Standards define establishment of:
  - Quality
  - Sec testing
  - Process improvement

International Organization for Standardization (ISO) has a lot of different standards.

ISO 9126:
- Defines best practices for software product quality
- Testing methodology focuses on:
  - Functionality: Accuracy, security
  - Reliability: Fault Tolerance
  - Usability: Learnability
  - Efficiency: Resource Utilization
  - Maintainability: Changeability, testability
  - Portability: Ease of installation to another system
  - NOTE: ISO 9126 has been replaced by ISO 25010:2011

Open Source Security Testing Methodology Manual (OSSTMM)
- Peer-reviewed system
- Provides a methodology for assessing operational security in 5 areas:
  - Data networks
  - Telecommunications
  - Wireless
  - Physical Security
  - Human

Software Engineering Institute of Carnegie Mellon University (SEI)
- Developed to improve the overall software dev process
- Ensure that software is dev'd in a secure way throughout the entire software dev process

### Testing Types

#### Penetration

Pentesting is active testing that involves the tester performing actions that a hacker would

- Actively trying to compromise the security of the system
- Different from most other testing that is passive
- Pentests are used to identify weaknesses within a system and PROVE that they are a threat
- Testers attack systems using:
  - Info obtained from the system
    - Recon, w/ tools
    - Perform a scan (`nmap`)
    - Normal usage
  - Knowledge of weaknesses of the system
- Test should replicate what an attacker would do IRL
- This locates vulns before a real attacker would


- Pentesting can detect vulns that are overlooked during dev't process
- Pentest can be white/blackbox testing
- General pentest steps:
  - Reconnaissance
  - Attack and exploitation
  - Removal of evidence
  - Create a report
    - Actions took
    - Success?


- Pentesting begins with specific objectives to explore
  - Input validation vulns
  - Configuration vulns
  - Host platform vulns during deployment


- Pentesting is slow and methodical
  - Most time spent planning and analyzing
- Errors and responses must be clearly recorded
  - Info obtained should:
    - Identify root causes
    - Lead to corrective actions

#### Fuzzing

Fuzzing is brute-force testing to test input validation issues.

- Injecting data to detect:
  - Which cause faults
  - Which are vulnerable to exploits

- Can be applied to all data exchange areas to ensure proper input validation:
  - Network, file, and web protocols can all be fuzzed

- Numerous web browser errors are identified by fuzzing

- Fuzz testing works well in all box testing methods

- Fuzzing is the best method to detect:
  - XSS
  - Injection

- Types of fuzzing:
  - Mutation fuzzing (dumb fuzzing)
    - Use known-good traffic and mutates it to create new input streams
  - Generation-based fuzzing (Intelligent fuzzing)
    - Uses input streams to determine the data streams for testing

#### Scanning

- Scanning is an automated form of detecting specific characteristics across:
  - Systems: OSes - OS fingerprinting
  - Applications: Weaknesses and vulnerability
  - Networks: Available Network devices
- Security scans are passive processes
  - It only collects information about the system, it does not attack systems
- Scanners can search for numerous specific conditions and compliance
  - OWASP/SANS top lists
  - PCI/Sarbanes-Oxley compliance (periodic security scans)

- Vulnerability scans
  - Goal is to detect weaknesses in software
  - Banner grabs (ask port 22 or 80 for banner)
  - Generates a report
    - And how to fix issues
  - Should be performed regularly

#### Simulation Testing

Idea that you want to create a mockup of the production environment where the app is going to be deployed once it is developed and completed.

Some issues we generally see are disparities between test/prod environments which can create problems in prod environments.

Simulation testing fixes this problem.

- Simulation testing used a mirrored production environment to test user applications for:
  - Config issues
  - Data issues

- Simulation testing is useful in locating issues before prod envs roll out.
  - Generally the last testing procedure prior to deployment

- Load testing is also performed to determine:
  - Acceptable availability
  - Performance

#### Failure Testing

FT is a form of test where the tester inputs incorrect values into the app to create conditions that will cause errors.

- Conditions resulting in incorrect values should be tested for:
  - Whether or not they cause errors

- Load testing is a common failure condition to test
  - Stress testing, break testing, fault injection
  - Lots of users connecting
  - Lots of data being sent to app
  - ...both?

- Determining how software functions under heavy load can:
  - Identify memory issues
  - Identify scalability issues
    - Can app handle growth of 10k to 100k users?
    - Load balancing solution

- Load testing is best performed early in the dev process

#### Cryptographic Validation

- Secure crypto is achieved by using:
  - Approved algos
  - Secure and correct impl't
- Crypto issues arise with:
  - Protecting keys and seeds
  - Proper operational conditions
  - Proper RNG
  - Key transmission

- RNG is best done through crypto libs
  - Don't make your own RNG

- Trusted crypto libs usually include a crypto RNG
- Key mgmt:
  - Don't hardcode keys within code
  - Keys should be generated and passed by reference
  - Key storage in memory should be noncontiguous

#### Regression Testing

- Regression testing validates for adverse impacts on software when changed by:
  - Configuration
  - Patching
  - New modules
  - Code changes

- As software progresses through multiple versions, regression testing becomes more tedious
- Software patch RT is very time consuming
  - Rerun previous test to ensure change has no negative impact
  - Create test for new functionality added

- RT approach should be tailored to the nature of change required:
  - Appropriate level
  - Appropriate breadth
  - Appropriate scope

- Specialized reports can assist in RT:
  - Delta analysis
  - Historical Trending

#### Continuous Testing/Continuous Test Driven Development (CTTD)

Continuous Test Driven Development (CTTD) is automatic execution of software testing.

- For apps under dev't, CT offers:
  - Realtime assessment of business risks

- CT allows for more informed decisions with respect to:
  - Release scope
  - Time
  - Quality

- CT provides:
  - Automatic, QUANTITATIVE assessment of risk
  - Actionable tasks to help mitigate risks

- Type of CT is Synthetic Transaction Monitoring.
  - A monitoring tool that emulates a visitor's visit to a site (via scripts). The STM monitors the transaction for performance and reliability.
- Constant application availability and performance
- Advanced app degredation warnings
  - You know before end users experience performance degradation

### Working with Test Results

#### Impact Assessment

Identifying bugs that have been discovered and then assessing bugs based on the impact they will have with application security.

Ex:
P1 - Critical
P2 - High
P3 - Medium
P4 - Low

- After bugs ident'd, they are recorded in a bug tracking system.
  - Which bug gets fixed next
  - When the bug gets fixed

#### Corrective Action

- Ideally all bugs would be fixed
  - Via the bug clearing process
  - At all stages of development

- Some bugs are too hard/expensive to fix
  - Effort/cost to fix outweighs potential risk
  - May decide to postpone bugfix to next app release

- A bug resulting in major redesign can be costly
  - If it is critical to the success of the system, it must be resolved

- Not all bugs are exploitable
  - If they are inconsequential, resolving it can wait until a future update

#### Test Data Lifecycle Management

- Testing can require specific data to perform certain tests:
  - Input test data and output test data
  - Testing for error conditions (error and exception handling)
  - Testing for correct referential integrity

- Test data must be created to duplicate:
  - Actual production data
  - Specific process conditions
    - i.e. data that fully activates all the logic of your process
- Real production data that's been anonymized can be used

## Software Acceptance, Deployment, Operations, Maintenance, and Disposal

### Software Acceptance Pre-release Activities

#### Pre-release Testing Process

Pre-release testing is a process that should be defined in the early stages of planning.

PrT is a process that has activities that encompass:
- Access
- Analysis
- Evaluation
- Review
- Inspection
- Testing
...of the app before it goes to production

- These activities should be performed in parallel


PrT objetive is to assess product:
- Accuracy
  - Does app do what it should do?
- Completeness
  - Is the app complete and performs all functions in full?
- Consistency
- Testability


PrT ensures product aligns with intended purpose within the target environment

- PrT:
  - Should be a continuous procedure
  - Is a key measure for software acceptance
    - Must be performed before software is presented to user base

PrT process consists of:
- Defining what is to be tested
  - Refer to requirement docs and design specifications
- Identify high risk characteristics in an app
- Identify testing priorities within application
  - What parts of app/types of testing are more important
- Determine scope and limitations of testing, approach, and methods
  - Black box? WB? GB? How will you test? What parts will you test?
- Determine testing env reqmnts
  - Hardware, software, and comms required to run app and test it
  - Comm devices/protocols needed on platform


PrT process consists of: 
- Defining testware
  - TW is software that can record and play back actions; this is used in the testing process, and in problem/bug tracking tools and coverage analyzers
- Test plan and specific test case creation and approval
  - Test case document - the expectation of a predictable response
- Testing result evaluation and reporting
  - Issues get tracked and fixes are recommended
- Retest as needed until bugs are fixed

#### Completion Criteria

CC:
- Defined by a project's contract
- Define a measurable outcome that can be objectively evaluated

- CC can be specific to a project

- Generic CC are available
  - Defined by ISO 9126


ISO 9126:
- Outlines 6 generic product completion criteria
  - Used to assess the product suitability after an app has been dev'd

- Functionality
  - Requirement suitability and output accuracy
  - Interoperability and functional compliance
  - Security

- Reliability
  - Maturity of product
  - Fault tolerance
  - Failure frequency
  - Ability to recover

- Usability
  - Ease of use
  - Comprehensibility

- Efficiency
  - Time and resource behavior
  - Processing and throughput

- Maintainability
  - How easy is it to make changes to the program?
  - Efficiency in addressing change requests
  - Product stability

- Portability
  - Product adaptability and extensibility

#### Risk Acceptance

- Risk properties to consider:
  - Safety and security requirements
    - Explicit and implicit (declarations vs assumptions)
  - Software complexity
    - To what degree is it complex
  - Performance and reliability factors


- Important to document risk identification and prioritization process


Ex: App is vulnerable to SQLi. It's a web app. The likelihood is high for that risk (sqli attack) and the impact is also likely very high.

RA addresses 2 questions:
- What is the likelihood of the risk occurring?
- What is the impact of the risk, should it occur?

- Responses to these two questions help identify acceptable and unacceptable risks
  - What has the highest probability of occurring
  - What will cause the most damage

- RA assessment should:
  - Use accepted and repeatable data collection methods
  - Produce accurate, independently verifiable evidence of the risks


- RA should be succinct with a defined scope and targeted on a specific threat
  - Utilizing an accurate threat model

- RA assessments:
  - Are a continuous process
  - Should maintain awareness of three critical assessment factors:
    - Interrelationships b/w all system assets
    - Specific threats to all assets
    - Specific technological and business risks of all vulnerabilities

### Software Acceptance Post-release Activities

#### Post-Release Plan

The goal of the PRP is the placement of the completed, tested application into a working environment.

Post-release activities:
- Performed by a separate (non-dev) deployment team
- Include a full audit of the installed configuration
  - This ensures software is installed and functioning appropriately
- Include capturing anomaly reports/change requests in the post-release plan

PRP outlines procedures for post-release process administration.

- Components of PRP can include:
  - Administration details of PRP process like:
    - PRP structure
    - Information flow
    - Mechanisms for adjusting app environment
    - Deviation policy (what?????)
    - Timing of reports
    - Problem resolution
      - If there is a deployment problem, there must be a process in place for handling those problems


Additional PRP components:
- Procedures for configuration auditing
- Baselining
- Problem reporting
  - Operation and mgmt aspects
- Mgmt plan revisions
  - Configuration and baseline changes

More PRP components:
- Anomaly evaluation process
  - To detect problems occurring within environment
- Change assessment process
  - Also a method of implementing change
- Level of status reporting
- Config mgmt processes for administering the environment
- Appropriate policies and procedures in place to determine what standards/convention should be followed with the implementation of the app itself

#### Validation and Verification

Validation: The software is meeting user-defined requirements
- Defines that the software is constructed properly

- Validation and verification activities are relevant to all stages of the software 
  - Activities performed are defined in the software validation and verification plan (SVVP)

- Validation and verification effort should be actively assessed
  - Determine if SVVP remains relevant

- The SVVP should define requirements for:
  - Resolution and reporting requirements for anomalies
  - Exception policy
  - Control procedures for baseline configuration and configuration mgmt
  - Guidance reqs for standards and conventions that should be used in the app environment
  - Reqs for documentation:
    - Plans, prodecures, results

- Validation and verification methods can typically be either:
  - Management -- Assess project suitability
    - Corrective actions, resource allocation, project scoping
    - Aim to uncover any deviation from agreed upon plans or procedures
    - Scheduled as part of initial project planning

  - Technical -- Verify software product itself
    - Does software product conform to specs, regulation, standard and plan adherence, accurate implementation?
    - Aim to uncover defects in software under modification or build
    - Scheduled as part of initial project planning


- Management V+V test may suggest corrective action and involve mgmt roles like:
  - Decision maker
  - Review leader and recorder
  - Technical and mgmt staff
  - Customer representative

- Mgmt reviews are scheduled as part of initial project plan
  - Also connected with project milestones

- Technical V+V
  - Completed my technical mgmt personnel and may involve managerial roles such as:
    - Decision maker
    - Review leader and recorder
    - Technical managers/staff
    - Customer technical staff
  - Evaluation of anomaly or defect impact

#### Independent Testing

Independent testing is similar to validation and verification, except that independent testing is performed by an independent, objective third party.

- Can also be called Independent Validation and Verification (IV&V)
- Ensures confidence, integrity, and trust in the product

- Third party requires flexibility to test and audit
  - Tests are based on the goals of the testing contract
- IV&V is credible when ownership of the product have no influence on the testers

- Regular audits should be performed
  - Auditing helps ensure conformance to regulations/standards

- Typical project audit targets:
  - Plans, procedures, contracts, reports, source code, and all deliverables

Once an audit is completed, all collected evidence is compiled into an audit report which includes:
- Initial conclusions
- Issues experienced
- Recommendations for correction and improvement

Once issues with the app are fixed, auditors will follow up on the implemented resolutions
- This ensures the reworked solution is completed properly

- Acceptance of the resolution results in the final audit report is required, and it outlines:
  - Who was audited: Organization
  - What was audited: Software
  - Why the audit occurred - Its purpose and scope


The final audit report, which includes acceptance of resolutions results, should outline:
- All relevant regulations and standards
- Evaluation criteria
- Anomaly classification
  - Minor or major
- Post-audit activity scheduling

### Installation and Deployment

#### Bootstrapping

Within the context of product/process deployment:
- Bootstrapping refers to operational correctness in both form and function

- Ensures a product will:
  - Launch properly
  - Function appropriately
  - Maintain expected accuracy

- This applies to any operationally self-sustaining function

- In software deployment, BS ensures accuracy of the initial config
  - This includes the app's default settings
  - Applies appropriate defaults
  - Sets proper execution parameters
  - Ensures accuracy of security features


- Within the context of product use, bootstrapping ensures continuous, effective operation by defining and deploying:
  - Rules
  - Policies -- Internal reviews
  - Practices -- Inspections
  - Measuring errors and correctness

#### Configuration Management Roles and Plan

- Typical installation and deployment config mgmt roles include:
  - Configuration manager
  - Baseline manager
  - Verification manager


- Config mgr
  - Ensures all change mgmt requirements are fulfilled
  - Documents software change requests
  - Manages software change authorizations
  - Verifies authorized changes are complete

- Baseline mgr
  - Creates and maintains a Baseline Mgmt Ledger (BML)
    - Includes all software config within a baseline
    - Pertinent devs can help
  - Responsible for all BML entries

- Verification mgr
  - Assures product integrity throughout the change mgmt process
  - Verify BML entries conform with identification scheme and changes have been completed
  - Perform milestone reviews and maintain review documentation
  - Must guarantee that the BML represents the current status of a product

- The Configuration Mgmt Plan should identify roles for:
  - Change mgmt
  - Baseline mgmt
  - Verification mgmt

- Should also specify the product identification number (PIN) and how it is set up.
- The Configuration Mgmt Plan (CMP) must be adhered to throughout the product lifecycle

- The CMP must also:
  - Specify/maintain a PIN
  - Define and assign roles for the configuration control boards
  - Establish mechanisms for timely information flow to managers
  - Outline method of change verification and validation
  - Itemize library maintenance rules
    - Dynamic libraries
    - Static libraries
    - Controlled libraries

#### Configuration Management Process

- Software products, systems, services, are made up of baseline config items.
- Each item is:
  - Identified
  - Defined
  - Formulated

The config mgmt process creates the foundational identification scheme for config mgmt. This involves each element in the system having its own unique identifying number.
- A big part of the config mgmt process is identifying the scheme for the number that represents these software elements


- The config mgmt process is made up of 6 process activities:
  - Process implementation
  - Configuration identification
  - Configuration control
  - Configuration status accounting
  - Configuration evaluation
  - Release mgmt and delivery


- Process Implementation
  - Major organizational activity
  - Requires formal strategic planning
    - Outlines activities, procedures, and schedule
  - Output of this activity is the 
    "strategic configuration management process lifecycle plan"
    - Comprehensive,
    - Accurate,
    - Well documented plan.


- Configuration Identification
  - This establishes the identification scheme to be used by all the software items within the baseline
    - Software items and baseline versions
  - Compiles formal documentation
    - For every item and associated version


- Configuration Control
  - Manages change -- Receives all product repair/change requests
  - Change requests are reviewed by a config control board
  - Audit trail is established
    - Why change occurred, who authorized the change


- Configuration Status Accounting
  - Tracks status and history of controlled software and baselines
  - Reporting covers project changes, software versions, and release attributes


- Configuration Evaluation
  - Evaluates to determine and certify product accuracy and functionality, based off of Statement of Work (SOW)


- Release Mgmt and Delivery
  - Formally controlled via library function
  - Code and documentation master copies maintained for life of product
  - Safety and security related materials follow organizational policies
    - As far as the handling storage, packaging and delivery

#### Release Management

Release Mgmt ensures:
- Accuracy of software products and updates
- The integrity of a software base config (baseline)

- Securely maintained via current baseline repository
  - Controlled check-out/check/in of baselines

- Baseline repos assist in VCS maintenance
  - Software deployments are associated with specific configurations
  - Releases are identified clearly to mitigate end user confusion

- Product baselines reside in specific repositories
  - Controlled repo: Contains official "live" version of each baseline
    - Access to these baselines requires authorization
  - Dynamic repo: For developer testing, to help fix/identify config anomalies, fixes, etc
  - Archive repo: houses latest baseline version
    - Strictly controlled access

### Operations and Maintenance

#### Monitoring

Monitoring's primary responsibility is to monitor the app within the target environment and ensure the app is functioning properly and performing well, and responding to incidents that occur that get picked up by monitoring tools.

- Ensures system security
- Optimal performance
  - In accordance with app plan
  - And review processes that are executed

We strive to have the application within operational assurance, this is why we monitor it.
- Needs to function, execute, and perform, as expected.

Effective operational assurance requires:
- That daily monitoring and reporting are part of overall assurance
- An enforcement policy that ensures the agreed monitoring and reporting methods for the app are being followed
- A formal problem resolution process

- Monitoring requirements require explicit identification during the requirements process
  - Service Level Agreement (SLA) or auditing requirements will define these:
    - What is logged - What data needs to be captured
    - Frequency of logging - How often logging will occur
    - How logs will be used - Data metrics

Effective mgmt needs measurement -- We need the data to see how the app is performing, and then make decisions based on that.

#### Incident Management

IM is monitoring and incident identification of incidents in the app env.
- Collection of objective, understandable data that can be actioned
- Timely incident analysis

Effective incident id'n assists in distinguishing:
- Code vulns
- Software exploit'n attempts
- User errors


Reporting and Control is a major component of incident mgmt.
- Incident reports should document all relevant aspects of an event
  - Event type
  - Potential impact of the event

ex: Password recovery attempts should be recorded, what was the impact?

- Incident reports should include all severity of events
  - Minor: Perceived insignificant/routine code defects
  - Major: Embedded trojans, backdoors

- Regardless of the incident, the appropriate, agreed-upon response must be initiated.
  - Typically involves incident response team
    - They respond to/evaluate the incident and determine what actions need to be taken after that.

- Anticipating incidents
  - Incidents are typically either:
    - Potential - Defects, unexpected flaws, without a present threat
    - Active - exploitation of unknown software flaws
  - Once vulns are understood, measures can be deployed
    - Patch or procedural changes

- Responding to active incidents
  - Appropriate rectification measures require immediate deployment
    - Handled by incident response teams to:
      - Mitigate damage
      - Monitor system modification/patch deployment to ensure fix works

- Establishing a structured incident response plan needs...
  - Preparation
    - Specific to organization
    - How to address passive and active incidents?
  - An organizational manual to guide incident response
    - Using standard practices
    - Include relevant terms and concepts
    - What role does each incident response team member have?
      - Very important point -- all members must understand what their job role actually is when an incident occurs and how they should respond.
  
- Ensure enough resources exist to handle the incidents
  - Resources allocated should be appropriate based on known incidents
  - Strike a balance between appropriate response and over-resourcing
  - Effective incident response policy will help identify the appropriate measured response

- Manage the incident response team properly
  - Team must be able to respond to all incidents without deploying unnecessary personnel
  - Team includes:
    - Manager, expert analysts, programmers, cybersec, computer crime specialists, legal, government experts

#### Problem Management

- Step 1: Determine if a problem actually exists
  - Can determine if it is a problem through replication and verification

- Step 2: Once a problem is identified, rectify the problem
  - Impact on all relevant items is assessed
  - Resources required to address the issue are estimated

- Step 3: Collect relevant details into a report
  - Analysis results
  - Implementation options

- Step 4: Report is escalated to a configuration control board
  - Approve or deny recommended modifications
- Upon approval, modification Statement of Work (SOW) is developed and communicated to interested parties

- Step 5: Modification implementation
  - A change manager ensures modification follows appropriate monitoring, control, and reporting procedures
  - Due care is required to avoid modifying any unaffected areas of the system

- Step 6: Review the system
  - Ensure system integrity is preserved
  - SOW was fulfilled

Finally: Once the modification is deemed correct, system reintegration can occur. This is putting the system back into its live environment.

Once we reintegrate, we verify it (make sure everything is operational) and final sign-off occurs

#### Change Management

All software modifications should occur under a change management process
- Upgrades, bug fixes, patches.

- Change management mitigates an organization's risk when modifying systems

- Software modifications frequently occur through vendors:
  - Patches
  - Hotfixes
  - Quick fix engineering (QFE)

- Patches can be regularly released OR on an emergency basis
  - Time depends on the fix
- Can occur as a single file or bundled into a 'service pack'

- Regression testing should occur for ALL patches and hotfixes
  - This ensures that the fix doesn't create new issues

- Organizational patch management integration with change mgmt is crucial for:
  - Controlled release/roll-out
  - Stability
  - Completeness

#### Backup, Recovery and Archiving

Backups are an important security measure when upgrading to a new production environment.

If something goes wrong with the upgrade process, you can restore to the original state.

- Software and data backup process involves a lot of effort and planning

- Must maintain archives of:
  - Early release versions
  - Associated datasets

- Important to consider security of archived backups
  - Encrypt it!

- Be aware of program and data retention cycles
  - A retention cycle is comprised of a complete set of backups to fully restore data
    - Full backup
    - Full plus incrementals
    - Be aware of incremental backups

- Retention cycles are stored as a group for a defined retention period

- Determining and managing retention cycles and periods need to be considered
  - Org specific
  - Legal, governance, compliance requirements

### Software Disposal

#### Software Disposal Planning

Software disposal is the removal of software after it is no longer needed by the org.

Can need removal because:
- License expiration
- Software is old
- Software is vulnerable

The software disposal plan should outline:
- Schedule of removal
- Actions that need to be taken
- Resources needed

With relevance to:
- Software services delivery termination
- Transformation of related systems
- Privacy and security health and safety in relevance to disposal actions

- Disposal strategy should be defined including:
  - Disposal constraints
  - Disposal activities
  - Transition periods
    - Need to make software replacement exists

- The software disposal plan should outline:
  - When and how support for the product will be withdrawn
    - This is very important
  - How all relevant product components will be archived
    - Make sure to do a complete backup of the software and its data
  - Responsibility for support issues
  - Transition details to replacement product
  - Accessibility to archived materials

#### Software Disposal Execution

SDE facilitates a smooth product retirement.

After you make your SD Plan, you want to put the plan in place to ensure software is retired in a smooth manner.

- End users require ample notice of product retirement plans and activities. This is important. They need:
  - Details of the replacement product
  - When it will be available
  - Why the product is being retired
  - Alternative support options

- User training is required to facilitate a smooth transition to the new product.

- Retirement notifications (with date) should be sent to all relevant users

- All relevant documentation should be archived
  - Logs or code

- All relevant data needs to remain accessible in the event of audit procedures
  - Data should be accessible

## Supply Chain and Software Acquisition

### Supplier Risk Assessment

#### Risk Assessment for Code Reuse

The CSSLP places a strong emphasis on risk assessment.

Risk assessment is really an info gathering process, used to:
- Identify and evaluate:
  - Threat impact
  - Appropriate threat response

It's important be be aware of all practical risks
- Risk certainty (will this happen?)
- Risk impact

Ex: A risk that's highly likely with low impact is not as much of a concern as a risk that's very unlikely with very large impact.

##### Machine level code reuse

Machine level code reuse (what is that??) has been common for years.

Machine level code reuse led to modern programming languages
- Standard Template Libraries (STLs)

The risk here is that there is an underlying flaw in the reused code.
It's easier to determine that if the code has been in use for a number of years, especially in STLs.

##### Application level code reuse

Application level code reuse involves repurposing existing components (source code) to build new software.

- This is cost effective
  - Saves time and reduces cost
  - Implies a measure of quality control
    - Meaning that, if you reuse shitty app level code, you'll get risk from reusing it.

It is harder to determine risk with app-level code.
- The app code is likely not used widely across the world (probably proprietary and only in org)
- Hasn't been field tested or analyzed
- There could be problems in code that you're not aware of.
- The act of modifying the code could exacerbate issues or make new ones.

Main risk with reusing components:
- Perpetuation of coding issues/bugs

Old code is NOT ALWAYS good code.

#### Code Reuse Plan Best Practices

The primary goal of a reuse plan is to:
- Collect and store secure, reusable components.

- Reused components require EXTRA assurance measures to:
  - Avoid perpetuation of coding issues
  - Validate proper functionality
    - Modularity, information hiding, decoupling
  - Confirm security

These reusable components need to have extra assurance because they will be widely used. 

---

Third party unknown source code reuse comes with extra risks.

- Open source code off the internet, like some nice Python package.
- Code we paid a vendor for (some DLL or something)
  - If it's a reputable vendor that's been in business for years,
    it's reasonable to assume that some level of quality control has 
    been effective within that vendor's dev team.
  - The bigger issue would be reuse contracts.

Liability in such cases should be clearly identified in reuse contracts
- Who is liable for risks brought upon by code issues? You, or the vendor?
- This is defined within the overall risk management plan

Reuse strategy identifies:
- Type of reuse required
- Appropriate measures for reuse planning

Reuse planning identifies:
- Usage of open-source code
  - This code was made by volunteers who just gave it away
  - ...heartbleed! Caused by open source technically.
  - You must assume that open source code has inadequate/absent testing, and that you can do the testing yourself.
- Any other reusable code
- Additional products and services

Reuse planning benefits the process by:
- Assessing advantages of reusable code
- Mitigating risk with reusable code

Code reuse risks need to be:
- Identified
- Documented
- Prioritized

The risks may be identified by how you're using the code. If the code that caused the heartbleed bug was only used in an internal application, versus being a cornerstone for secure comms with many webservers.

After identifying the risk, you must document and prioritize those risks.

The risks are then disseminated to stakeholders.
- May be business unit managers, software engineers, vendor representatives.

Now we can all talk about and evaluate the risks.

This validates the reuse process, and is not meant to prevent code reuse.
By mitigating the risks, you validate the whole issue of reusing code.

- This should occur as an iterative process
  - You cannot take 1 look and identify all issues and answer all questions

#### Intellectual Property

Products that are difficult to protect are easily stolen.
- DVDs
- Music

Most software products/components are difficult to protect.
- If a third party company got access to a large web app you owned, and they used modules from it, would you even know?

Many software products and components suffer from this risk.

- Organizations should implement measures to mitigate intellectual property theft

IP theft can be:
- Plagirism
- Duplicating products
- Copying ideas

IP can be difficult to quantify due to its abstract and intangible features.

Software IP can include:
- The code itself
- The processes you utilize
- The algorithms (unless well known/widely used)

IP property theft can be mitigated through:
- Auditing
  - Audit anyone who has access to your code.
- Inspection
  - If third party vendors are coming onto your premises who might be exposed to IP, you can inspect the portable media that they might bring.
- Penalties
  - Can include litigation

It's important to increase awareness of possession rights throughout the supply chain.

Federal regulations, acts and laws define terms for unauthorized software/IP:
- Renting
- Leasing
- Lending
- Copying
- Some exceptions exist for creating backup copies

#### Legal Compliance

- Compliance refers to conformance with and satisfaction of relevant laws and regulations
  - All software within a supply chain must adhere to this

- Regulatory requirements can originate from varied sources:
  - Industrial
  - Government
  - Contractual

- Compliance should be considered throughout all aspects of the supply chain
  - This includes third party vendors

Example: If you're in healthcare, it's not enough to just be HIPAA and HITECH compliant, but are all of your vendors also HIPAA and HITECH compliant?

- Noncompliance can result in negative organizational repercussions:
  - Damage to reputation
  - Financial loses
  - Legal/criminal actions

Compliance requires appropriate reporting methods.

Reporting should capture objective details which support adherence to regulatory requirements
- Details should be auditable
- These reports are given to regulatory bodies as evidence of compliance

- Regulatory bodies can differ
- Compliance requirements between them can confict or be similar in scope

The best way to do this is to have a single organizational process that's responsible for compliance with all regulations.

#### Supplier Prequalification

- Prequalification solidifies a supplier's trustworthiness
- Occurs through a certification process

- Buyers are often unaware or not in control over supplier products
  - Supplier Prequalification:
    - Mitigates purchaser risk
    - Reveals development methodologies

- Supplier security considerations are paramount in supply chain mgmt
  - Example: The famous Target breach came through a 3rd party vendor that was working with Target, and they got compromised. That compromise was used to get CC# info from Target.
  - Provides reliability and integrity of information and communications technology (ICT)

- It is difficult to ensure ICT with commercial, off-the-shelf (COTS) products
  - If you're a small business, you may have no way of leveraging the vendor to give you details.

- Buyer should consider:
  - Capabilities of vendor/contractor
    - Contractors often overstate (lie) about their capabilities
    - Determined via historical work
  - Validation of subcontractor trustworthiness


- Third-party entities offer certification or prequalification validation
  - i.e. ISO Common Criteria evaluation
- Provides objective validation of a supplier's trustworthiness
  - Independent of geographical location
- Provides evaluation of supplier capability
  - Identifies measures for improvement if necessary

### Supplier Sourcing

#### Supplier Sourcing Challenges

Finding the right supplier can be a challenge.

These challenges are very important specifically when dealing with software security.

- All supply chain sources should be scrutinized for trustworthiness

- Analysis and understanding of each supplier's operation will determine the best fit for outsourcing

- Activities of all contractors and subcontractors should be clearly defined:
  - Who is performing what work elements in appropriate contractual language

Strategic security is also a concern, if you're outsourcing to a foreign country.
- Political positions of foreign entities may impact supplier trustworthiness
  - Intentional security flaws?

The degree of foreign organizational involvement requires clear identification
- This is an entire risk management process in and of itself
- Clarifies risk management requirements and direction

This is a serious issue if you're a defense contractor. If you're doing something like making videogames, etc, it's not as big of an issue.

Industry sourcing models can help rank suppliers.
- Rankings set up on risk vs return are very common.

Ranking criteria are identified and scored, like:
- Technical aptitude
- Alignment with business objectives
- Resource challenges (big issue, often contracting companies don't start out with enough resources)

Security trade-offs should be weighed:
- Strategic improvements vs. maintenance of operations:
  - Strategic improvements have a higher risk when being outsourced
  - If you are maintaining a low-priority operation, there is less risk when outsourcing that task
  - Operational modernization can increase risk
  - Improvements must be justified
- High vs low risk:
  - Risk must be balanced against ability to effectively manage it

Impact of one supplier on another:
- The introduction of suppliers to the supply chain can impact security

Opportunity costs:
- Long term investment must be measured against risk and potential opportunity loss

If you have different suppliers in your supply chain, one can actually impact the security of others, especially if there's a progression (TODO what is a progression??)

Let's say Supplier A provides some component to Supplier B who assembles a piece of equipment to be supplied to you. Now, supplier A can negatively impact B or positively impact it.

Outsourcing is going to give you risk. But, if you don't outsource, do you lose opportunities?
- Are there capabilities that you can outsource and add to your business?
- Can you increase productivity?
- All of these have to be weighed against the risks.

The best organizational sourcing decision will be arrived at once these tradeoffs have been appropriately vetted.

Look at opportunity losses/gains, risk versus reward analysis, basic business analysis.

#### Contractual Integrity Controls

Very important part of supply chain management.

You have to have processes in place throughout your supply chain that are controlled by your contracts.

These contractual integrity controls are meant to ensure that the contract's obligations are met, and they have to be included in all supply chain management.

Development processes throughout the supply chain are controlled with contracts
- Legal binding agreements

Contractual integrity controls must be included within supply chain management
- Criteria specification
- Evaluation criteria
  - How, when, and where will you validate the vendor is actually fulfilling those contractual obligations?

Integrity controls ensure:
- Appropriate levels of product assurance are carried out
  - Testing
  - Audits
- Appropriate issue resolution measures
  - For noncompliance
- Implementation and monitoring of corrective actions
- Something in place to resolve any issues discovered and confirm that issues were resolved

Administratively, Contractual Integrity Controls assist in contract fulfillment
- Configuration mgmt

Formal contract configuration management throughout the supply chain should be well:
- Maintained
- Controlled
- Coordinated

This lets you make sure that the contracts with all vendors at every step in the cupply chain are being fulfilled.

This formal contract configuration management should be maintained.

This results in total control of product integrity, by taking control of your supply chain.

#### Vendor Technical Integrity Controls

The goal of technical controls are to affirm underlying components of a product.
- Assuring all components are:
  - Complete
  - Correct
  - Consistent

We need to make sure that a product AND its subcomponents meet our requirements.

Vendor technical integrity controls support:
- Overall software integrity throughout the supply chain
  - Product baselines and repositories
- Timely dissemination of contractual requirements to subcontractors

You have to make sure of everyone in the supply chain, including subcontractors, are aware of the contractual requirements.

Vendor technical integrity controls should include testing and auditing measures
- Contract security testing
  - Validation of security assurance aligning with contractual stipulations
  - Ensuring appropriate verification and validation throughout supply chain
  - Compiling and disseminating reports

Integrity controls have to align with formal baseline practices
- Modifications and releases are carefully recorded, documented, and reported


Deployment of technical controls provide a framework for correct technical methods (FOR VENDORS) in:
- Software engineering
  - Code structure, commenting and variable naming conventions
- Testing practices
  - Reuse
- Data structure use (xml? json?)
- Resource management

Basically, common commenting, code structure, code quality, code TESTING, data structure standards.

Three testing activities help validate accuracy of technical processes:
- Unit testing
  - Occurs within the coding process
- Integration testing
  - Occurs within software integration process
- Qualification testing
  - Occurs at acceptance checkpoints

#### Managed Services Controls

Managed Services Controls have similar requirements as technical controls, and the goal is the same.

You have a vendor that provides some service/product, and you need to ensure contractual obligations are met.

- Utilizes a continuous assurance approach
  - You need to have ongoing assurance that all of the contractual obligations are met
  - Contract specifies requirements precisely

- Assurance requires specific details of services
  - ...So that the customer receives what they requested

Service requirements should be specific outlining:
- Formal terms
- Contractual terms

ex: It's not enough to say that "The vendor will respond to an issue within 3 business days".
What is a "response"? Is it a call back? Or does it mean having someone on-site attempting to fix an issue? Or does it mean the issue is fixed?

Your managed services control must be encompassing.
- Covering everything you expect to be performed
- What must be performed?
- How do I evaluate if the work was accomplished?
- What are remediation methods if a standard wasn't met?
  - Steps?
  - Punitive issues?
- What is "good service"?
  - Time limit threshold on calls?
  - Good review threshold?

#### Service-level Agreements

SLAs outline performance expectations between suppliers and customers.
- Legally binding and made by lawyers (most of the time)
- Details product or service requirements/performance
- Details method to determine if agreed upon service/product has been provided
  - What are the metrics to measure the performance?

The SLA contract:
- Defines observable behaviors used to identify performance levels
  - Objective criteria! Numbers!!
- Specifies criteria to measure behavior accuracy within deliverables

"Was it delivered adequately?"

The SLA contract also addresses:
- Usage
  - How long can I use this product? How can I use it?
- Ownership
  - Do I own the software? Any/all of the data?
- Licensing
  - If you outsource DB Mgmt, and it needs Oracle DB, who pays for the licenses and owns them?
    - Do you or the vendor own it?
- Warranty
  - Is there a warranty with this product?

- Due to the complexities of software, performance adherence is often evaluated by a third party.
  - The mechanisms for which must be included within the contract

- SLA contract modifications require legal advisement
  - Changes to metrics, requirements, penalties, etc.

- Maintenance or small "changes" are the customer's responsibility and don't need attornies
  - i.e. software version upgrades

### Software Development and Testing

#### Technical Controls

Security for your code repo:
- Is your code base public vs private facing?
- Authentication
  - Not everyone should make changes to code
- Auditing
  - What changes were made by what user at what time
- Access Control
  - Not every user can change every bit of code
- Protection from destruction
  - Very few people should be able to delete code, esp. in OSS
- Rollback

Build environment security:
- Who can build?
- Principle of least privilege

#### Code Testing and Validation

This is not about testing code to see if it's up to spec.

Also not about seeing if it's secure code.

This is about specific things that occur when code is checked into and out of source control, or when different developers work on it.

Backdoor detection:
- Or, when auth controls can be circumvented. Not always intentionally done.
- Prevent and detect with:
  - Code inspection
  - Code comparison
  - Code review checklist
- A good policy is to review checked in code, or do random weekly/monthly reviews

Embedded malware detection:
- Doesn't mean that it's malware in your code, but code that launches some other embedded module.
- Source code analysis
- SCA integrated with build
- Antimalware

#### Security Testing Controls

Security Testing Controls are an important part of the entire software dev process and also have to be included in the various aspects of your relationship with vendors within your supply chain.

Security Testing Controls are methodologies where you test the security of your systems, including Software Systems and also those provided/supported by third party external vendors.

The goal of this is to affirm security and integrity across the entire supply chain for the product being implemented and utilized and the processes.

- Assurance methods for security must be contractually defined
  - Implement comprehensive assurance procedures
    - Evaluate likelihood and impact of associated risks, i.e. Lower priority for unlikely risks

Sound security testing controls are formed through 8 steps.

1. Initiate a process
  - Define key security roles. Who does what? What part of the security process does your organization do?
2. Identify security testing issues throughout every level of the supply
3. Create a basic security testing plan
  - Define and identify standards, practices, and audit/control activities
4. Implement a security testing process
  - Also, conduct security training for personnel
5. Assign roles and responsibilities, and schedule jobs
6. Monitor activity definition and reporting
7. Identify security issues and resolutions
8. Periodically evaluate the testing process to ensure optimal performance and viability

All of this is meant to conform to contractual security, which assumes that your contract has security objectives, requirements, and roles clearly defined.

#### Software Requirements Verification and Validation

Verification and validation of software requirements is also an important aspect of software security.

This is something a manager would do, not a software engineer.

Objective:
- Verify product complies with contractual requirements
- Assess all functional and nonfunctional requirements

Software must comply with all:
- Contracts
- Requirements
- Standards
- Plans

This process also applies to subcontractors.

All the contract requirements should be given to the subcontractors. This is very important.

- Timely dissemination of contract requirements
  - Ensure subcontractors have a clear understanding!!
- Final deliverables must meet contract requirements
- Appropriate contractor oversight
  - Typically product-specific

Subcontractor oversight ensures all product components meet specifications
- Hardware
- Systems
- Software
- Service stipulations

The supplier is responsible for:
- Verifications
- Validations
- Tests

### Software Delivery, Operations, and Maintenance

#### Chain of Custody

Chain of custody is a concept related to forensics and evidence, and it can be applied to software development.

CoC provides:
- Continuous awareness of product baseline state
- Monitoring methods
- Error mitigation
- Cost reduction

In software dev, it means accounting for software from the moment of design to the moment of deliverable.

It also provides overall insight of product evolution throughout the supply chain.

All changes and transfers made throughout the product lifecycle must be:
- Authorized
  - There should never be a change made that was not authorized by a responsible party. 
- Transparent
  - It should be documented and verifiable that a particular change was made, why it was made, who authorized it, and the result of the change.
- Verifiable

This leads us to Configuration Management, which:
- Maintains integrity of all configuration items
- Allows for evaluation of changes
  - Was it effective?
  - Did it lead to other issues?
  - ...

Other benefits of Software Configuration Management:
- A basis to measure quality
- Improves overall software dev't/maintenance
- Simplifies testing and quality assurance
- Mitigates product release management errors
  - ...by addressing problems early in the process to manage changes
- Allows you to track components in the software
- Streamlines change mgmt and problem tracking


Software configuration management leverages three aspects of the software development lifecycle process (SDLC)
1.  Development
    - Identification process
2.  Overall configuration management
    - Authorization and configuration control
3.  Software Quality Assurance (SQA)
    - Verification

#### Publishing and Dissemination Controls

How do we control the publishing and dissemination of the software?

These issues are all about ensuring a secure transfer, for customers and throughout the supply chain.

Publishing and dissemination controls help prevent counterfeit products.

Ex: `nmap` tool. OSS that had a version distributed that was bundled with malware! This is an instance where the publishing and dissemination of a product was not tightly controlled and led to counterfeit products.

Publishing and dissemination controls can be in the form of:
- Product licenses
  - Certification of product authenticity
- Encryption
  - Mitigate exploits like MiTM
  - Activation should take place over encryption
  - Contextual metadata
- Digital signatures
  - Pub Key transfer, checksums
- Tamper resistance methods
  - Internal
  - External

#### System-of-Systems Integration

A SoS is a system whose components are other systems.

Your org may have a system that's responsible for placing customer orders, maintaining inventory, etc.

You might have a vendor who has their own system, and you choose to integrate these systems in order to make the ordering and processing go smoothly.

The goal of Systems Integrations is to bring these systems together so they cooperate just like a larger system.

Individual component systems of a SoS are relevant to supply chain problems.

Each of these systems have their own security issues, their own risks, and you have to manage the systems individually and in the aggregate.

The integration of components require:
- Supply chain mgmt
- Assurance of integrity
  - Data integrity
  - Sending data
- Assurance of effectiveness of concurrent processing
  - Each component does its own processing
  - Is it effective?

Who's responsible for systems integration?
- Systems engineers!

Systems Engineering is concerned with the design, understanding, and testing of larger complex systems.
- Focus on SoS applications:
  - Identify the roles of each system
    - Data format
    - Comms channels
  - Characterization
    - Can you characterize...
      - Security
      - Functionality
  - Conceptualization
    - What are you trying to achieve?
    - What should each subsystem do?
  - Analysis
    - What does the supersystem do?

Analysis of SoS applications uses methods like:
- Continuous system modeling
  - This models the continuous processes of all these systems working together
- Agent-based modeling
  - Modeling each individual component agent (TODO what is this?)
- Unified Modeling Language (UML) can be used


The analysis is based on some basic design principles:
- Abstraction
  - Abstract the concepts away from the implementation
    - i.e. for data storage, we care about security and efficiency, we don't care about the database format or protocol.
- Modularity
  - Each module is a separate subsystem and they are individually contained, module boundaries are well-defined.
- Information hiding
  - No individual component should reveal more information than is absolutely necessary for the other component.

This is relevant to all levels of design, in 1 system or a SoS.

#### Software Authenticity and Integrity

This is related to publishing and dissemination integrity.

All about ensuring that the software used is really the software you expect it to be.

It doesn't only have to do with piracy, it's also about trojan horses -- Adding malware to software.

- Supply chain interface authentication
- Integrity assurance

The final software product must be disseminated in a manner that ensures authenticity AND integrity.

- Software authenticity requires nonrepudiation of source
  - Endpoint authentication
    - Kerberos
  - PKI
    - i.e. sign the software with a PrivKey and verify with PubKey.

Integrity is different from authenticity. 

Does the software do what it is supposed to? Is it error-free? Is it performant?

Common methods of integrity checking:
- Testing
  - Static checks
  - Dynamic checks
- Auditing (at the code level)
- Targeted bench checks
  - Executing code at a dev's machine and seeing how it performs
- Reviewing


Counterfeit components within a supply chain can be hard to detect if they're done well. This can threaten the integrity of the entire system.

It is a business liability and sometimes vendors don't update counterfeit components.

Countermeasures:
- Trusted, vetted repository (foundry) for executable code.
- Having direct control over the supplier.

It is easier in business-to-business environments to control software integrity and authenticity than it is in widely distributed end user software.

#### Product Deployment and Sustainment Controls

Product Deployment and Sustainment Controls are about having control over the way your product is deployed, and how you sustain the software through config mgmt, updates, and patches.
- Ensures software changes do not compromise system integrity
- Allows for evaluation and performance of changes
- Manages how changes are developed, tested, and deployed.
- Provides a foundation for quality measurement
- Simplifies testing and QA methods

Configuration management processes support product deployment and sustainment controls through:
- Configuration control
  - Configuration must be secure
- Verification control
  - Verify upgrades for appropriateness, compatibility with other software, and OSes.
  - Custom code extensions
    - Some products allow extensive mods, like scripting.
    - This has to be controlled as it could lead to security or performance issues

All of this falls under Operational Readiness, ensuring your product is ready for operation in all the environments it needs to work under.

#### Monitoring and Incident Management

An incident is ANYTHING that interrupts normal operations.
- Malicious actions
- User error
- Power issues

Incident mgmt ensures organizational incident response integrity through:
- Monitor systems
  - You can't respond to an incident if you don't know about it.
- Analyze the incident
  - And what caused it
- Response
  1.  Contain the incident
  2.  Fix the problem
  3.  Restore things to normal operation
  4.  Determine why the incident occurred

Any event an organization deems as harmful will initiate an incident response.
- Timely responses can only be accomplished with appropriate monitoring.

The entire supply chain should be monitored, (somewhat) manually.
- Reviews
- Inspections

Example of an incident response:

-   A hacker with an external IP is attacking a router connected to WAN to get into your system.

    You have a IDS/IPS behind a firewall which is after the router.

IPS/IDS are examples of automated monitoring systems.
IDS will only detect intrusions, and IPS will detect and prevent intrusions by blocking traffic.
However, IPS false positives may negatively affect traffic.

The incident response if relevant to every single event that occurs.

It could be an event you predicted, or not (like a data breach).

1. Set up protection for the event.
2. Detect the event.
3. Respond to the event.

The difference in response lies in whether or not it was a planned/unplanned event.

Your response should be planned even if the event was not: What do we do if a hard drive craps out? Or there's a virus?


#### Vulnerability Management, Tracking, and Resolution

Vulnerability mgmt is about:
- Identifying what components in your supply chain have vulnerabilities
- Identification and repair of components in the supply chain
  - Code modules
  - Identified through software assurance (i.e. extensive testing)

- Patching identfied vulnerabilities
  - Disseminate the patch to all involved parties and make sure it's applied
  - Exploit prevention and mitigation
  - **Timely** release and application of patches is important

All patches and repairs require:
- Planning!
- Tracking!

You can't indiscriminately deploy patches without planning and testing. Example: CA eTrust Antivirus, LSASS service.

When you have a new patch, install it on a TEST system to make sure it doesn't screw anything else up.

Track the entire process, and make sure approved patches are applied and there is a way to roll back the process.

- Resolution efforts gain efficiency with appropriate control
  - This means timely, effective resolution management of all vulnerabilities.

The entire supply chain requires thorough investigation and identification for vulnerable components.

After this, you have to alter the vulnerable component and repair it under **coordinated supervision**.

### Supplier Transitioning

#### Code Escrow and Data Exports

##### Code Escrow

Code Escrow is storing source code with a trusted third party.

An Escrow Agent will store the code in a secure place until release is needed.

Example: A vendor creates software products and sells them to customers.

The vendor is responsible for updating, maintaining, patching the code.

What happens to the customers if the vendor is either no longer willing or able to support their product?
1.  Vendor discontinues the product: No updates or patches :(
2.  Vendor goes out of business: No updates or patches :(
3.  Code Escrow: The Escrow Agent can release the code to the customers if 1 or 2 happens

Factors to consider with Escrow Agreements:
- Subject and scope of the escrow
  - What is stored and how?
- Conditions for release
  - What must happen for the Agent to release the source code? (sauce code is extremely valuable)
- Non-compete clauses
  - The Agent must not be involved in developing similar software
- Bankruptcy issues
  - If the software is in an Escrow but the initial vendor declares bankruptcy, the bankruptcy court may get involved because that software is an asset that may need to be liquidated to pay the debts of the company.
- Open Source issues
  - OSS maintain escrows, primarily to maintain integrity of the software.

##### Data Exports

- Exporting data backups
  - What format will you export the data in?
    - Is it compatible with the system importing the data?
- Data formats
  - A lot of systems use widely used formats like XML, CSV, or JSON.
  - Healthcare: HL7.
- System integration
- System communication

#### Contracts

Software contracts are important to software security.

They are generally a legal agreement between the software producer and the consumer.

Contracts cover:
- Requirements for software
  - What does it do?
- Regulations and standards applicable to the project
  - May include Software Dev Methodologies
- Software development
  - Testing, etc
- Security Code Reviews
  - Do your own reviews.
- Code Ownership
  - Who owns the code?! Very important.
  - What does ownership mean?
  - What can be done with the code?
- Acceptance criteria
  - What must be met in order for the customer to accept the software?
- Certification & Accreditation
  - Must be detailed in contract.

Contracts usually cover IP rights.
- Provide protections for components that may originate at lower levels of the supply chain
- This mitigates copyright violation and piracy.
- Patents, Copyrights, Trade Secrets, Trademarks must be addresses.

Consequences of disclosure must be defined in the contract.
- Confidentiality agreements
- Noncompete agreements
- Identify confidential information
- Accidental disclosure
- Deliberate disclosure
- Forced disclosure, i.e. a court forces you to disclose
  - You generally want to immediately notify the owner of the confidential information that you've been subpoenaed and may need to disclose confidential information. That gives the owner an opportunity to contact their attorney.
- Already publicly disclosed
  - Is no longer confidential information.

The best advice is to consult an attorney who specializes in these matters and make sure these issues are clearly defined in your contracts.