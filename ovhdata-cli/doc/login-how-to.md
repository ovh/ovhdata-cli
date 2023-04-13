# To login into OVH Data Processing platform, you need OVH API credentials.

## How many times should I login?

You should login once per region you want to use. If you want to see your current region among all the available ones just type *ovhdata-cli config ls*.

If you want to switch to a new region type *ovhdata-cli config set \<config-name\>* and then login to that region if needed.

If you want to change your current user you don't need to *logout*, just do another *ovhdata-cli login* with the new credentials to use on the current region.

A browser is open follow the form in order to create the application with at least this given parameters:
- GET/*
- PUT/*
- POST/*
- DELETE/*

Once the API keys are generated, enter them with the CLI.  
