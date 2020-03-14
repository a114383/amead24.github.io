# RPC

Remote Procedure Call - Common set of operations be accessible over a network
    How are these different than APIs?
        futher reading: https://www.smashingmagazine.com/2016/09/understanding-rest-and-rpc-for-http-apis/
          dissertation on rest: https://www.ics.uci.edu/~fielding/pubs/dissertation/fielding_dissertation.pdf
        *  "An API is built by defining public methods; then, the methods are called with arguments. RPC is just a bunch of functions, but in the context of an HTTP API, that entails putting the method in the URL and the arguments in the query string or body."
        *  The author uses REST endpoints that look like `POST /trips/123/start` which seem possible, but my first thought would be to create an endpoint using a query string.  Again these might just be implementation details, but compared to RPC this would be `startTrip("?trip=123")`.
            * More importantly RPC isn't there to describe a resource (REST exposes a class or mental model) for users to interact with, while RPC traditionally is only for executing a select few tasks.  Meaning you're probably not creating a rideshare app using RPC, but maybe only a select service is RPC.
            
