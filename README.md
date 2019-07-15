# shorty-rs

Shorty is a simple URL shortening (micro?)service, created initially for our internal use at Moving Gauteng.
The first version was written in JavaScript [https://github.com/MovingGauteng/shorty](https://github.com/MovingGauteng/shorty).
The current version is written in Rust.

## Rewrite

Why rewrite it? This needs some backstory. Moving Gauteng is a public transit info website. Think of Google Maps' transit data, except as a browsable catalogue in a website.
We (royal 'we' because I'm the only software person in our house) initially wrote shorty because we own a domain called `rwt.to` (meant to be 'route to', from when I was trying out an A-B thing), and we had a need for sharing shorter URLs on social media.

Moving Gauteng is a pastime thing, it sucks a lot of time and money from us (we haven't made a cent from it), so when things go down, I lately have little time to see what's wrong.
I've found good success with porting a lot of our backend stuff to Rust. Our biggest success is a service called `vehicle-streams`, that powers [https://movinggauteng.co.za/explore](https://movinggauteng.co.za/explore). If you visit this URL you might have to pan/move the map to Johannesburg, South Africa, and it's best viewed during the GMT+2 04:00 to 21:00 (the page's incomplete though).

Anyways, a few months ago I realised that URLs shared from our site weren't being shortened anymore. So, shorty-js was down. I thought restarting the service would work, but after a few times, nothing gave. I didn't have time to look into it, so I thought I'd save it with the long list of "some-days".

I finally had a few hours to look at it, and instead of debugging it, I thought I'd just rewrite it in Rust, so I'd **never have to look at it again**.

This is the rewrite.

## Why not use the thousands of existing services?

1. I don't like external network calls to someone else, and don't like that dependency
2. We had the domain and weren't actively using it

## How can I run it?

Just use Docker. We've added a multistage `Dockerbuild` for convenience. It creates a container that's 6.33MB large.

Otherwise, your preferred Rust workflow is fine, if you're a Rust user. This will run on stable.

## How does it work?

It is a gRPC server, that has 3 endpoints:

1. shorten an URL (with some optional Google Analytics utm_ things)
2. retrieve a shortened URL
3. increment URL counter arbitrarily (should probably merge this with 2 above, happy to get PR 🤔)

You configure your custom `short enough` URL through environment variables, then run the thing, and forget about it.

## Architecture

### Data Model and Storage

shorty-rs, like her predecessor, ([un]fortunately) uses MongoDB. A port to Postgres or something else would be quick to do.
*If you wish to volunteer that, please hide it behind a `#[cfg]` thing. I don't mind if that becomes a default feature. MongoDB works well for us, so we'd like to still use your improvements*.

The data model is loosely like:

```
shortenedurl (
  id: string, // prefer BSON ObjectId with 24 characters, but UUID v4 should work)
  url: string, // the url that shorty creates
  original: string, // the original url that user supplies
  constructed: string, // the constructed url that is returned, with utm_ parameters
  ga_campaign: [GoogleAnalyticsCampaign], // see below, you might create a separate table for this, or add it to this one
  created: timestamp,
  accessed: timestamp, // last accessed date
  visits: integer, // incremented each time URL is accessed
)

// GoogleAnalyticsCampaign
(
  utm_source: string,
  utm_campaign: string,
  utm_medium: string,
  utm_content: string,
  utm_term: string
)
```

### gRPC

We use [tower-grpc](https://github.com/tower-rs/tower-grpc). It performs very well.

## Performance

It uses **2MB** of RAM under load, I've done a bit of stress testing in the 3 hours that I wrote it, but I don't mind a few people hitting the service to try bring it down (please let us know if you do, so we can monitor what's happening).
I normally use opentracing on our rust services, but I opted out this time, so I can't really say how fast it runs. It does a fairly simple job, so who cares?

## How do I redirect URLs?

This is the part that we've left out for now, for now.

Write a service (or add an endpoint to your existing service) that takes the shortened URL, and redirects the user to their desired destination. Here is an example in NodeJS Express framework.

```javascript
  app.get('/:shorty', function (req, res, next) {
    if (req.params.shorty.match(/([0-9a-zA-Z]{5,10})/)) {
      Shorty.GetUrl({
        url: `https://rwt.to/${req.params.shorty.trim()}` // replace with your domain
      }).then(data => {
        // thisi is why I'm saying better to make this endpoint redundant
        Shorty.AddCounter({id: data.id, value: 1}).catch(err => {});
        return res.redirect(data.url);
      }).catch(err => {
        return res.status(500).send('unable to find link');
      });
    } else {
      return res.status(500).json({error: 'incorrect parameters'});
    }
  })

```