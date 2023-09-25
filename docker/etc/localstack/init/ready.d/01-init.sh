#!/bin/bash

# Create files for test
mkdir -p /tmp/01-init.sh/
echo "hoge" > /tmp/01-init.sh/hoge.txt
echo "fuga" > /tmp/01-init.sh/fuga.txt
echo "vaa" > /tmp/01-init.sh/vaa.txt

# Create bucket
awslocal s3api create-bucket --bucket sample-bucket
awslocal s3api put-bucket-versioning --bucket sample-bucket --versioning-configuration Status=Enabled

# Check bucket and objects
awslocal s3api list-buckets
awslocal s3api list-objects --bucket sample-bucket

# Put objects
awslocal s3api put-object --bucket sample-bucket --key hoge.txt --body /tmp/01-init.sh/hoge.txt
awslocal s3api put-object --bucket sample-bucket --key fuga.txt --body /tmp/01-init.sh/fuga.txt
awslocal s3api put-object --bucket sample-bucket --key vaa.txt --body /tmp/01-init.sh/vaa.txt

# Check objects and versions
awslocal s3api list-objects --bucket sample-bucket
awslocal s3api list-object-versions --bucket sample-bucket

# Delete object
awslocal s3api delete-object --bucket sample-bucket --key fuga.txt

# Check objects and versions
awslocal s3api list-objects --bucket sample-bucket
awslocal s3api list-object-versions --bucket sample-bucket
