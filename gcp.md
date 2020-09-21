https://googlepluralsight.qwiklabs.com/focuses/10758295?parent=lti_session
=> Objectives

1. Create cloud storage bucket
2. Review File Upload UI and code changes
3. Write code to store uplooaded file data into cloud storage


Process:
1. Activate gcp cloud shell
2. Clone the github repo with: `git clone https://github.com/GoogleCloudPlatform/training-data-analyst`
3. Change your working directory by pasting the following on your cmdline: `cd ~/training-data-analyst/courses/developingapps/nodejs/cloudstorage/start`
4. Type `ls` to view the files and folders available in this folder.
5. Run `. prepare_environment.sh` to execute the prepare_environment script
6. Run `npm start` to start the application
7. To view the application, click on the `web preview` and then `preview on port 8080` option resulting from your previous action.
8. stop the running application on the `cloudshell` using `Cmd+c` 
9. Create cloud bucket storage witsssxh the format `<whatever-id>-media` using the following command: `gsutil mb gs://$DEVSHELL_PROJECT_ID-media` (where $DEVSHELL_PROJECT_ID is already available as an environment variable and refers to your gcp project id)
10. Export the cloud storage bucket name using the following cmd: `export GCLOUD_BUCKET=$DEVSHELL_PROJECT_ID-media`
11. Confirm that the environment variable has been created using `echo $GCLOUD_BUCKET`, this should output text ending with `-media`
12. 
13. Click `Open editor` on the cloudhell (we would be editing the files in our cloned folder)
14. Navigate to the `cloudstorage file` name app`cloudstorage.js` which contains the gcp configuration by doing the following:
        - click on `training-data-analyst`
        - find and click on `courses`
        - find and click on `developingapps`
        - find and click on `nodejs`
        - find and click on `cloudstorage`
        - find and click on `start`
        - find and click on `server`
        - find and click on `gcp`
        - find and click on `cloudstorage.js` file

15. Declare and define the constant `Storage` as shown below under the first `//TODO` comment in this file:
    const Storage = require("@google-cloud/storage");
16. Add this after the second `// TODO` comment:
    const storage = Storage({
        projectId: config.get("GCLOUD_PROJECT")
    })
17. Add this below the third `// TODO` comment:
    const GCLOUD_BUCKET = config.get('GCLOUD_BUCKET');
18. Add this below the fourth `// TODO` comment:
    const bucket = storage.bucket(GCLOUD_BUCKET);

19. Add the following within the `sendUploadToGCS` after the declaration of `oname`

```
const stream = file.createWriteStream({
    metadata: {
        contentType: req.file.mimetype
        }
    });

    stream.on('error', (err) => {
        next(err);
    });

stream.on('finish', () => {

    file.makePublic().then(() => {

      req.file.cloudStoragePublicUrl = `https://storage.googleapis.com/${GCLOUD_BUCKET}/${oname}`;


      next();
    });
  });
  stream.end(req.file.buffer);
```

20. Click on `File` and then `Save` to save your changes
21. Start the app with `npm start`
22. download the image file here to your local machine: https://storage.googleapis.com/cloud-training/gcpdev/lab-artifacts/Google-Cloud-Storage-Logo.svg
23. Click on the `web preview` option and then `preview on port 8080` option to load the new page
24. Answer the questions as stated below

25. confirm that the object was added to he entry using this command: 
`gsutil ls -r <gcp-bucket-name>-media`
you should see the response containing the name of the image you uploaded earlier as the suffix of the response.

25. Add /api/quizzes/gcp to the end of the application's URL.
26. Return to the application home page and click the Take Test link.
27. Click GCP, and answer each question.