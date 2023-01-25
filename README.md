# pareto-hockey-populate

Toolkit for friend's in 'Pareto Hockey' project to populate database of NHL career data from EliteProspects API

### Intro

EliteProspects is an online data aggregation and visualization service for NHL prospect and career data. 

They provide a public-facing API with several endpoints to access and download this data for personal use which can be found [here](https://app.swaggerhub.com/apis-docs/esmg/Eliteprospects/1.0).

However, performing certain big aggregations over their dataset can prove awkward and time-consuming. Many endpoints return redundant information and it's difficult to aggregate over the specific ways we are looking for.

So to perform analysis, we temporarily cache the data in a local database (so we can query using SQL). pareto-hockey-populate creates and populates the local cache of needed relevant data.

### Usage

(Incomplete, not ready for use)
