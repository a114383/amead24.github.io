# Site Reliability Engineering

### Logging & Monitoring

* One of the biggest takeaways from this section was moving to visualizing metrics rather than focusing on raw statistics.  One example was binning latency for a histogram so you could better understand the tail metrics rather than focusing on averages/medians.

* Seemingly counter-intutive they suggest focusing on symptoms rather than root cause solutions, unless the cause is very imminent.  This one I understand from a large team or organizational perspective, where everything is a service, but on a small team spending significant amount of time putting out fires is my least favorite part of the job.  If the cause is outside my immediate control reasonable there's little to do, however if it suggests a larger refactor is necessary then finding the balance between fixing it now versus smaller fixes I would normally suggest the minimizing of context switches is more important (for me at least).

* "Fears against short term fixes suggest a lack of confidence in managing technical debt" - Well that's spot on.


### Autonomous Systems

1.  Extendability
2.  Centralized Mistakes - Solve once, solve everywhere
3.  Increased MTTR & Decreased Cost to repair
4.  Save Time Long Run - xkcd/1205

* The example of the fail-over database highlights the idea that rather than shipping a database with manually scripts to run, you should ship a platform that handles these itself.  There's constant talk throughout the book about when alerting someone if there's a manual action to take, you should target those for automation first.  Requiring a human in the loop to click a button is an ideal indicator for automation.

* "Requiring idempotent fixes meant teams could run their 'fix script' every 15 minutes without fearing damage to the cluster's configuration." Here's the real amazing feat to me, creating idempotent piplines is a lot harder than it sounds, but makes iteration significantly easier and less risky.  Even running tests becomes dangerous if you're not setup properly which tends to defeat the whole purpose!

Automation Metrics:

1.  Competence - Accuracy
2.  Latency - how quickly all steps are executed when intiated
3.  Relevance - Increase/decrease of real world process automated

* "A PM not affected by low-quality automation will always prioritize new features over automation."  There is a signifcant push & pull between taking time to step back and avoid new features until everything has been completely automated, and business wanting new features whom interact with PM/management who doesn't run the automation.  This has historically been the bigger pain point and there seems to be little suggestions in solving this dilema.

* Chapter 7 concludes with another story about idempotency.  **Note to self, read a lot more on how to effectively do this.**

