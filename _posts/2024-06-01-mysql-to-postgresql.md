---
layout: post
title: Zero Downtime MySQL to PostgreSQL migration
tags: devops postgresql
category: 
---

In my first year working at Woovit as a back-end developer, I had a hard time. We had MySQL, ElasticSearch and DynamoDB. This stack came up during a growing stage of the company, but now there were less developers than DB flavours. Letting aside the cognitive overhead of working with 3 differents DBs (not counting Redis), the objective problems we faced were always related to data syncing across the DBs. When I've realized that we could replace 3 different DBs by PostgreSQL, while keeping all product features, this migration became a personnal goal. 

After a few months trying to convince the team that this move was a real win, they agreed to make the migration. For several reasons, we decided that we would migrate with zero downtime. In this article I will focus on the MySQL to PostgreSQL migration of a django app since it is the most delicate part of this whole process. Moving the existing features from ElasticSearch and DynamoDB to PostgreSQL is an interesting topic about which I will hopefully write in another post.

# Zero Downtime Migration of a Django App

To my surprise, I've not found a solution to easily migrate a django app from MySQL to PostgreSQL with no downtime. Basically one needs:
- MySQL to PostgreSQL replication working during the whole process
- old MySQL django app version working until B
- new PostgreSQL django app version working from time A, where A<B
- a load balancer to distribute requests so that both version can work at the same time

Additional steps:
- During the time between A and B, we have actually two master nodes. To avoid the most common source of data inconsistency, we offset all serial PKs in the PostgreSQL instance by a sufficient large number.
- Django make a great job by abstracting the underlying DB implementation. Almost no modifications were needed in the python code, but there are some differences that we need to take into account in the replication process. For example, Django uses a `big int` to represent duration in MySQL, while it uses a proper `interval` field in PostgreSQL. Actually, this step became the most complex part of this project, since I found no tool that would perform the replication with an additional transformation for some fields.

# Available tools
## AWS DMS solution
I will not enter in details here. I just want to mention that giving it a try was a waste of time, and probably you want to skip it also.

## [pgloader](https://pgloader.io/)
This is a tool made by the author of the excellent book [The Art of PostgreSQL](https://theartofpostgresql.com/). Although it does not replicate from MySQL to PostgreSQL, it does support data transformation. The tool is made in Common Lisp and the user specifies data transformation in Common Lisp. I was impressed by how easy the language allows for dynamical extensions. It was my first experience with Common Lisp and it was quite easy to implement the required transformation. The tool uses PostgreSQL `COPY` commands to load the data into PostgreSQL and it is quite fast. If you don't need online replication, this is the way to go.
I've used this tool in a first step in order to have a fully working django app based on PostgreSQL. In this step, I've found many data inconsistencies in our source DB that PostgreSQL would not tolerate.

## [PG chameleon](https://github.com/the4thdoctor/pg_chameleon)
PG chameleon is a python-PL/SQL project that implements logical replication from MySQL to PostgreSQL. The python part fetches data from MySQL and send as JSON records to PostgreSQL. Then the PL/SQL part applies the data modification statements inside PostgreSQL.
It solved 95% of our problem. We still needed to transform some `big int` fields in MySQL to `interval` in PostgreSQL.

## [PG chameleon hackish fork](https://github.com/rafonseca/pg_chameleon/tree/django-migration)
Inspired by PG loader, I've extended PG chameleon so that we can define transformations for some specific fields. In this fork, I've only implemented the django duration field required transformation. It does not have the same extensibility as PG loader, so we need to modify the code if a different transformation is needed. I should do a proper PR with extra docs one day.


# Result
After a couple of weeks of testing we were ready to the actual migration. It was a susprisingly smooth deploy. 2 hours from the deploy we were still waiting for the first bug. Only then we realized that it was a real success. I cannot say that it was 100% because a few weeks later we found a minor bug in one of our TypeScript services (point for Django as a good ORM here).

When we had MySQL, I was always reluctant to implement features and business logic inside the DB. First, because MySQL did not provide enough resources. Then, I knew that keeping everything inside Django would make this eventual migration easier. After this point, I became confident in implementing things in the ~~DB side~~ PostgreSQL side. It was a turning point in my dev career. PostgreSQL quickly become my tool of choice instead of python.


