# Backend Gilion case

At Gilion we get access to multiple companies' sensitive data such as revenue, operating expenses and so forth. We extract companies' raw data no matter if it comes from Stripe, Paypal, Klarna etc. and transform it into a standardized format that we can then use to render graphs for these companies.

## The task

You are tasked with building the backend application responsible for serving this data to a frontend dashboard or other backend services. Below you can find some general information and expectations on the application.
We’ve provided a producer which can be run through docker. The producer should not be changed in any way, and should just be used to get the data.


- We'd like to be able to retrieve the data, through an API, the data belongs to company AI Quantum Innovations co.

- We prefer the api to have a clear and clean architecture.

- We'd like to serve the data in a good format, and we'd like the frontend to be able to stream it.

- We'd like to be able to also retrieve the metric `C` which is calculated by taking `A` divided by `B`.

- We’d like to be able to retrieve the metric ‘D’ which is calculated by taking the average of (A_{kn +1},…,A_{kn +10}), basically the moving average of A.



## The format
The data is sent in byte chunks. Each chunk is 6 bytes, the first 4 bytes is the date (YYYYMMDD), the following two bytes are the numbers A and B respectively.

## Instructions
To launch with docker-compose, ensure you have it installed, then cd to this directory and do `docker-compose up --build`
