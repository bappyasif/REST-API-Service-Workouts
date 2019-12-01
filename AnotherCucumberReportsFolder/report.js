$(document).ready(function() {var formatter = new CucumberHTML.DOMFormatter($('.cucumber-report'));formatter.uri("Include/features/Books Requests/Books.feature");
formatter.feature({
  "name": "Updating A Book Contents",
  "description": "",
  "keyword": "Feature",
  "tags": [
    {
      "name": "@tag"
    }
  ]
});
formatter.scenarioOutline({
  "name": "Updates Books",
  "description": "",
  "keyword": "Scenario Outline",
  "tags": [
    {
      "name": "@tag1"
    }
  ]
});
formatter.step({
  "name": "\u003cBook_ID\u003e And \u003cBook_Title\u003e Book Obeject Properties Are Available For Update Request",
  "keyword": "Given "
});
formatter.step({
  "name": "All Object Properties Are \u003cBook_ID\u003e \u003cBook_Title\u003e \u003cBook_Description\u003e \u003cBook_Excerpt\u003e \u003cPublished_Date\u003e Ready To Use For Making This Update API Request",
  "keyword": "When "
});
formatter.step({
  "name": "I Verify REST API Update Request Status Code \u003cCode\u003e",
  "keyword": "Then "
});
formatter.step({
  "name": "I Also Checkout Some Custom Keywords Usage Examples",
  "keyword": "Then "
});
formatter.examples({
  "name": "",
  "description": "",
  "keyword": "Examples",
  "rows": [
    {
      "cells": [
        "Book_ID",
        "Book_Title",
        "Book_Description",
        "Book_Excerpt",
        "Published_Date"
      ]
    },
    {
      "cells": [
        "011",
        "Updated Title",
        "Updated Book Description text. Updated Book Description text.",
        "Updated Book Excerpt text. Updated Book Excerpt text.",
        "2019-11-25T14:02:37.311Z"
      ]
    },
    {
      "cells": [
        "012",
        "Another Title",
        "Another Description Updated Text. Another Description Updated Text.",
        "Another Excerpt Updated Text. Another Excerpt Updated Text.",
        "2019-11-29T14:02:39.311Z"
      ]
    }
  ]
});
formatter.scenario({
  "name": "Updates Books",
  "description": "",
  "keyword": "Scenario Outline",
  "tags": [
    {
      "name": "@tag"
    },
    {
      "name": "@tag1"
    }
  ]
});
formatter.step({
  "name": "011 And Updated Title Book Obeject Properties Are Available For Update Request",
  "keyword": "Given "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.bookDetails(int,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "All Object Properties Are 011 Updated Title Updated Book Description text. Updated Book Description text. Updated Book Excerpt text. Updated Book Excerpt text. 2019-11-25T14:02:37.311Z Ready To Use For Making This Update API Request",
  "keyword": "When "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.updatingBookContentsResponse(int,String,String,String,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "I Verify REST API Update Request Status Code \u003cCode\u003e",
  "keyword": "Then "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.verifyStatusCodeResponse(String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "I Also Checkout Some Custom Keywords Usage Examples",
  "keyword": "Then "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.customKeywords()"
});
formatter.result({
  "status": "passed"
});
formatter.scenario({
  "name": "Updates Books",
  "description": "",
  "keyword": "Scenario Outline",
  "tags": [
    {
      "name": "@tag"
    },
    {
      "name": "@tag1"
    }
  ]
});
formatter.step({
  "name": "012 And Another Title Book Obeject Properties Are Available For Update Request",
  "keyword": "Given "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.bookDetails(int,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "All Object Properties Are 012 Another Title Another Description Updated Text. Another Description Updated Text. Another Excerpt Updated Text. Another Excerpt Updated Text. 2019-11-29T14:02:39.311Z Ready To Use For Making This Update API Request",
  "keyword": "When "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.updatingBookContentsResponse(int,String,String,String,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "I Verify REST API Update Request Status Code \u003cCode\u003e",
  "keyword": "Then "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.verifyStatusCodeResponse(String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "I Also Checkout Some Custom Keywords Usage Examples",
  "keyword": "Then "
});
formatter.match({
  "location": "UpdatingBookContentsRequest.customKeywords()"
});
formatter.result({
  "status": "passed"
});
});