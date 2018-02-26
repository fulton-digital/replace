# Replace

A simple templating tool designed to allow for simple find/replace on template files. This was originally conceived to aid in templating kuebrnetes config files.

## Example

You have a config file

```
db_url={{ DATABASE_URL }}
db_user={{ DATABASE_USER }}
```

and this file differs between dev, staging, prod etc. Replace will help you generate a config file with the proper variables replaced via the command line. For example to generate a file for this template you can run.

```
replace -t template.yml -r DATABASE_URL=gcloud/mysql/db1 DATABASE_USER=app1
```

This will output a template to std out on the CLI for use. If you miss any replacements or do not provide the proper replacements this tool will let you know what replacements are needed to be provided.

If you want to output to a file just use the -o flag

```
replace -t template.yml -r DATABASE_URL=gcloud/mysql/db1 DATABASE_USER=app1 -o output.properties
```
