## 2012 (beginning of highschool)

### NCP Tech Club

Part of a student-led school-wide computer repair club at Northside College Preparatory High School. Serviced laptops, servers, and desktops.

### Landscaping: Dirt Actualizers

Worked for “Dirt Actualizers”, a landscaping club at Northside College Preparatory High School.

Programmed an arduino + electrodes to detect moisture differences in dirt to be able to tell when a plant needs watering.

### OBLand/HenryLand: Self-hosted minecraft server

I ran and administered a few minecraft servers with friends.

![A picture of my very old Minecraft server listing.](/static/projects/henryland.png)

<https://www.planetminecraft.com/forums/minecraft/servers/henryland-92154/>

I think I was about 13 years old at the time. So! In 2012 (and in 2008, when my father showed me how to replace RAM), my love for computers, and Linux, started.

I learned how PHP code works, although I didn't understand what "code execution" was, I was smart enough to figure out that configuration files were how Minecraft plugins communicated with a person's web browser.

I found out how to port forward, figured out how to install plugins to a Minecraft server. I used the WAMP stack: Windows, Apache, MySQL, PHP. I learned how to use MySQL, and I learned how to use PHPMyAdmin. Again, I had no real clue what I was doing, but through trial and error, I figured it out.

We served about 20 individual players during 2012. I think that's a pretty good number for a 13-year-old. I was proud of it, and I still am. I think that it's a great example of how a young person can learn how to do something, and then do it. If they're dedicated enough, they can learn anything.

## 2013

## 2014

## 2015

## 2016

## 2017 (end of highschool)

### ASCII Duplet compression algorithm

Over the summer, I enrolled in an IIT summer Mathematica course where I coded an ASCII compression algorithm that took 256 of the most common duplets of characters in an ASCII file and compressed them into a file containing a dictionary followed by compressed data.

I'm actively searching for this code. It's possible it's lost to the sands of time, but I'm sure that I can recreate it if I need to.

The algorithm worked by scanning a file and identifying the most frequent two-character combinations (duplets). These duplets were each assigned a single byte (essentially creating a custom 256-entry dictionary). The file was then rewritten using the custom dictionary, replacing common duplets with their corresponding single-byte code. The compressed file consisted of the dictionary itself, followed by the transformed data stream.

This approach was a primitive form of dictionary compression, loosely inspired by how algorithms like `LZ77` replace recurring patterns with shorter references. It was fun, if not hard, to make.

#### Simulated recreation in Python

##### Code
```py
from collections import Counter

# Example file contents
ASCII_FILE_CONTENTS = "Mary had a little lamb, little lamb, little lamb, Mary had a little lamb, its fleece was white as snow. And everywhere that Mary went, Mary went, Mary went, everywhere that Mary went, the lamb was sure to go."

# Step 1: Find all duplets (overlapping pairs of characters)
duplets = [ASCII_FILE_CONTENTS[i:i+2] for i in range(len(ASCII_FILE_CONTENTS) - 1)]

# Step 2: Count how often each duplet appears
duplet_counts = Counter(duplets)

# Step 3: Extract the 256 most common duplets
most_common_duplets = [pair for pair, _ in duplet_counts.most_common(256)]

# Step 4: Build a dictionary mapping each common duplet to a byte value (0-255)
duplet_dictionary = {duplet: chr(i) for i, duplet in enumerate(most_common_duplets)}

# Step 5: Encode the file using the dictionary (replacing each matched duplet)
compressed_data = []
i = 0

while i < len(ASCII_FILE_CONTENTS) - 1:
    pair = ASCII_FILE_CONTENTS[i:i+2]
    if pair in duplet_dictionary:
        compressed_data.append(duplet_dictionary[pair])
        i += 2
    else:
        compressed_data.append(ASCII_FILE_CONTENTS[i])
        i += 1

# Handle the final character if the file has an odd length
if i < len(ASCII_FILE_CONTENTS):
    compressed_data.append(ASCII_FILE_CONTENTS[-1])

# Final compressed output: dictionary + compressed data
compressed_file = {
    "dictionary": duplet_dictionary,
    "data": "".join(compressed_data)
}

# Example output
print("Original length:", len(ASCII_FILE_CONTENTS))
print("Compressed length (excluding dictionary):", len(compressed_file["data"]))
print("Dictionary (first 10 entries):", dict(list(duplet_dictionary.items())[:10]))
```

##### Output

Notice how the compressed length is roughly half the original length, thanks to the replacement of common duplets with single-byte codes.

Also, notice how "ry" and "Ma" are among the most common duplets, as they appear frequently in the text.
"ry" is replaced by the byte `'\x02'`, and "Ma" is replaced by the byte `'\x05'`.

We also see "ar" being replaced by `'\x06'`, "y " by `'\x07'`, and so on.
This is actually inefficient, as we could have used a more sophisticated encoding scheme to save even more space.
It's inefficient because of the overlap between the most common duplets. If a duplet half-overlaps with another, that's wasted space.
`"Ma" "ar" "ry"` getting encoded is worse than just `"Ma" "ry"`, for example. We could save space by not encoding "ar".

ALSO, another limitation - we can't use the first 256 ASCII characters in the dictionary, as we need them for the dictionary itself. We could have used a more sophisticated encoding scheme to avoid this limitation, or we could have used code-switching to switch between different dictionaries, or switch between a dictionary and a different encoding scheme.

> (Henry thinking: Does LZMA do this? I think it does. I should read the implementation and dissect a running LZMA implementation's byte stream to see how it works.)


```txt
>>> # Example output
>>> print("Original length:", len(ASCII_FILE_CONTENTS))
Original length: 208
>>> print("Compressed length (excluding dictionary):", len(compressed_file["data"]))
Compressed length (excluding dictionary): 104
>>> print("Dictionary (first 10 entries):", dict(list(duplet_dictionary.items())[:10]))
Dictionary (first 10 entries): {'e ': '\x00', ' l': '\x01', 'ry': '\x02', ', ': '\x03', ' w': '\x04', 'Ma': '\x05', 'ar': '\x06', 'y ': '\x07', 'it': '\x08', 'le': '\t'}
```

## 2018

### Logisim ARC Tutoring

[==> See this link for source code! <==](https://github.com/meltingscales/cs-350/tree/master/Logisim)

#### Summary

I tutored many students in digital circuit design using a simulation software called "Logisim".

I worked at the Academic Resource Center (ARC) at IIT. I helped students with their homework and projects, and I also created a few circuits of my own. I also taught C code and assembly in a CPU OPCODE language called "LC3".

![A picture of LogiSim and some circuits - NAND and full adders.](/static/logisim.png)

### Homeless Shelter Donation Coordinator

[==> See this link for source code! <==](https://github.com/meltingscales/homeless-shelter-donation-pickup-coordinator)

[==> See this link for drawings! <==](https://github.com/meltingscales/homeless-shelter-donation-pickup-coordinator/tree/master/_sketches)

#### Summary

This was a project that I worked on that was inspired by my mother, Anne-Marie Keswick. She worked at various different women's shelters and homeless shelters in the Chicago area.

The project was a web application that would allow people to donate items to homeless shelters. The shelters would have a list of items that they needed, and people could sign up to donate those items. The shelters would then be able to coordinate with the donors to pick up the items.

It was planned to orchestrate logistics and optimize the distribution of desired goods, weighted by category and urgency.

Its design was supposed to be similar to Kubernetes, except decentralized with a focus on local communities. Copying nature is always a good idea, and the way that ants and bees work together is a great example of how to distribute resources efficiently.

I only got rough sketches done, but I think that the idea is still a good one. I hope that someone else can pick up where I left off and make it a reality.

## 2019 (beginning of college)

### OSINT Dashboard: Mary "Bowser"

[==> See this link for source code! <==](https://github.com/meltingscales/ITMS448-osint-dashboard-Bowser)

#### Summary

An OSINT dashboard for gathering intelligence from multiple sources.

The project's name is Bowser, named after [Mary Bowser](https://en.wikipedia.org/wiki/Mary_Bowser).

We are focusing on domestic terrorism.

We scraped 4chan and Reddit using their publicly available APIs.

We used old NSA ECHELON/PRISM lists of keywords that could indicate domestic terrorist/hate crime activity. This is a naive approach, but I feel that our code can easily be adapted. [Our "`ContentFlagger`" class can be easily adapted to use different types of classification methods](https://github.com/meltingscales/ITMS448-osint-dashboard-Bowser/blob/cb25a756e7b59990813ac3b85f42b48a7a44c9a6/bowser/contentFlagger.py#L49).

##### `ContentFlagger` modular code
```py
class ContentFlagger:
	"""A class that can flag content as containing specific words or phrases."""

    # ...
    # See this `flag_content` method - a naive approach using keyword matching![![alt text](image-1.png)](image.png)
	def flag_content(self, content: str) -> bool:
		"""Apply this ContentFlagger's analysis to some content. Returns T/F"""

		if content is None:  # thanks for the null values json <3 i feel like i'm in Java all over again
			return False

		# check all our keywords
		for word in self.keywords:

			# If we don't care about case, lower everything.
			if not self.keywords_case_sensitive:
				word = str(word).lower()
				content = content.lower()

			if word in content.split(" "):  # split by spaces -- word must be padded by spaces
				return True

		# check all our regex
		for rexp in self.regex_matches:
			if re.compile(rexp).match(content):
				return True

		return False

```

### Improving Incident Response of the American Red Cross in the Greater Chicago Area by Using Text Classification of Posts From Twitter

[==> See this link for a PDF presentation <==](/static/IPRO%20-%20Improving%20Incident%20Response%20of%20the%20American%20Red%20Cross%20in%20the%20Greater%20Chicago%20Area%20by%20Using%20Text%20Classification%20of%20Posts%20From%20Twitter.pdf)

#### Summary

This project developed an innovative tool to enhance the American Red Cross’s emergency response by leveraging social media data, specifically by mining and classifying tweets related to house fires in the Chicago area. By integrating a Twitter scraper that filters posts based on keywords and filters by geolocation, with a suite of machine learning models—including Naive Bayes, Logistic Regression, and notably Linear Support Vector Classification—our team demonstrated an effective way to sift through vast amounts of social media content. This approach enables the rapid identification of critical incidents such as house fires and floods, significantly reducing the need for manual data review.

This tool and the associated machine learning models could be used in many different ways:

- To enrich an existing data feed
- As a standalone tool for incident detection
- As a way to provide real-time data to emergency responders
- As a way to provide real-time data to the public or citizen scientists
- Any other way! Go clone our repo and try it out! 😁

The analysis revealed that while the automated system is promising—accurately detecting disaster-related tweets and providing a streamlined process for incident reporting—there is still room for improvement. Future enhancements, such as deeper integration of the scraper with the classifier and the implementation of active learning strategies, could further refine accuracy and reduce false positives. Overall, this project lays a strong foundation for harnessing real-time social media data to support timely and informed emergency response efforts, paving the way for smarter, data-driven disaster management solutions.

Ideally, Twitter should provide these sorts of feeds to emergency responders, but until then, we have to rely on our own tools. Social media is a powerful tool for emergency response, and we hope that our work can help save lives and reduce the impact of disasters. 🌍🔥🌊

## 2020

### Vulnerability "Field Guide" - CWE Reference Manual

I created a "field guide" that provides a comprehensive overview of common vulnerabilities and their remediation strategies. This guide is intended to serve as a reference for security engineers and developers, helping them to quickly identify and address vulnerabilities in their code.

It was over 20 pages, covered hundreds of different CWE types, and was a great resource for our team. I believe that it is still being used by my team today, in 2025.

## 2021

## 2022

### A survey on automated software vulnerability discovery, exploitation, and patching - NYU-CS-GY-6813 - Infosec and Privacy

This paper is a survey of the current state of automated software vulnerability discovery, exploitation, and patching. It covers the history of automated vulnerability discovery, the current state of the art, and future directions for research.

Its main conclusion is that solely human-powered vulnerability discovery is no longer sufficient to keep up with the pace of software development. Automated tools are necessary to keep up with the pace of software development and the increasing complexity of software systems.

It seems that the growth of vulnerabilities is hyper-linear, and that the number of vulnerabilities is growing faster than the number of software developers. This is a worrying trend, and it is likely that the number of vulnerabilities will continue to grow in the future.

[==> See this link for source code! <==](https://github.com/meltingscales/NYU-CS-GY-6813-infosec-and-privacy/tree/master/paper/final)

<iframe width="560" height="315" src="https://www.youtube.com/embed/BgICl-5bqbQ?si=QxhW5DyURWL57YWK" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>


## 2023

### Vulnerabilities in Open-Source Language Servers - NYU-CS-GY-6233 - Intro to OS

[==> See this link for source code! <==](https://github.com/meltingscales/NYU-CS-GY-6233-intro-to-os-final-paper)

<iframe width="560" height="315" src="https://www.youtube.com/embed/msj1KjN7eb8?si=AnzAFE22jZzZWmiT" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## 2024

### Hidden Vulnerabilities in Google Play Store Apps - NYU-CS-GY-9223 - Mobile Security

[==> See this link for source code! <==](https://github.com/meltingscales/NYU-CS-GY-6xxx-mobile-security-final-project)

[==> See this link for a PDF presentation <==](https://github.com/meltingscales/NYU-CS-GY-6xxx-mobile-security-final-project/blob/master/MobSec%20Final%20Presentation.pdf)

<iframe width="560" height="315" src="https://www.youtube.com/embed/4o2URWPtqX4?si=p-6e_cmdP7jCyWri" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

#### Summary

This is a survey of 10 semi-randomly selected apps from the Google Play Store. We found that all of the apps had at least one vulnerability, and that some of the vulnerabilities were severe. We also found that there were some slight accuracy issues with free SAST/Secrets tools.

We used simple, free, open-source tools to extract the APK, and decompile them. We also used a free SAST tool to scan the source code for vulnerabilities.

## 2025

### Homeless Shelter Donation Coordination System - New York University - CS-GY 6083 2025 Spring Database Final Project

This was my final project for my database class at NYU. It was a project that I worked on that was inspired by my mother, Anne-Marie Keswick. She worked at various homeless shelters in the Chicago area. The project was a web application that would allow people to donate items to homeless shelters. The shelters would have a list of items that they needed, and people could sign up to donate those items. The shelters would then be able to coordinate with the donors to pick up the items. It was planned to orchestrate logistics and optimize the distribution of desired goods, weighted by category and urgency.

It'd be neat to combine this with my really early project, the Minecraft server, and have a Minecraft server where people could pay to donate items to homeless shelters. I think that would be a fun project. A closed-loop ecosystem that uses Minecraft to guide donations to homeless shelters.

I'd also love to combine the really old code from late highschool and some of my drawings of the homeless shelter donation system.

I did not implement all of those features, but I did implement a basic version of the system. I think that the idea is still a good one. I hope that someone else can pick up where I left off and make it a reality.

[--> This is my final presentation video <--](https://youtu.be/0jUU-g4dFVg)

[--> This is my final presentation PDF/ppt <--](https://github.com/meltingscales/cs-gy-6083-2025-spring-db-final-project/blob/master/PRESENTATION-Homeless-Shelter-Donation-Coordination-System.pdf)

[--> This is the source code for the project <--](https://github.com/meltingscales/cs-gy-6083-2025-spring-db-final-project/)

<iframe width="560" height="315" src="https://www.youtube.com/embed/0jUU-g4dFVg?si=6GtBMQnaMg54NcjO" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

### Legal Discovery: Google Chrome History

As an assistant to a friend who was a lawyer, I provided assistance as a neutral investigative party to prove whether or not
an individual accessed certain documents from their web browser during a period of time.

I had no connection to the individual, and my lawyer friend had no incentive to find or not find evidence, so
I feel I was not influenced to perform with bias of any kind.

I scanned browser log files in Windows, sqlite databases, and exports from the browser directly.

To the legal counsel, I provided a log with timestamps of what data I accessed, my notes, and when I accessed the data.

I used a specialized open-source tool meant to read the browser history sqlite database, and I also read it directly
with SQLite Browser.

All in all, it was a fun engagement and I could probably easily automate it since it was just a data analysis job!

### C.A.R.E. Animal Shelter Donation Aggregation tool

I volunteer at a wonderful animal shelter called "Community Animal Rescue Effort" (C.A.R.E.) in Skokie.

https://www.carenorthshore.org/

Their staff needed a more efficient way to stitch data together, so I made them a website. It's pretty secure, fast, and simple.

https://care-shelter-donation-aggregation-163012697625.us-central1.run.app/about

https://github.com/meltingscales/CAREShelterDonationDataAggregation
