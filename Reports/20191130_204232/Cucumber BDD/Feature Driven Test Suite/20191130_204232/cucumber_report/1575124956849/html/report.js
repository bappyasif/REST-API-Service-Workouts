$(document).ready(function() {var formatter = new CucumberHTML.DOMFormatter($('.cucumber-report'));formatter.uri("G:/Katalon Studio/Katalon Workspace/REST API Service Workouts/Include/features/BDD.feature");
formatter.feature({
  "name": "REST API Requests Example",
  "description": "  I want to use this template for my feature file",
  "keyword": "Feature",
  "tags": [
    {
      "name": "@tag"
    }
  ]
});
formatter.scenarioOutline({
  "name": "Post A New Book Object",
  "description": "",
  "keyword": "Scenario Outline",
  "tags": [
    {
      "name": "@tag1"
    }
  ]
});
formatter.step({
  "name": "\u003cBook_ID\u003e And \u003cBook_Title\u003e Book Obeject Properties Are Available For Example",
  "keyword": "Given "
});
formatter.step({
  "name": "All Object Properties Are \u003cBook_ID\u003e \u003cBook_Title\u003e \u003cBook_Description\u003e \u003cBook_Excerpt\u003e \u003cPublished_Date\u003e Ready To Use For Making This POST API Request",
  "keyword": "When "
});
formatter.step({
  "name": "I Verify REST API Post Request Status Code \u003cCode\u003e",
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
        "Feature Title",
        "Feature Description Sample Text",
        "Feature Excerpt Demo Text",
        "2019-11-25T14:02:37.311Z"
      ]
    },
    {
      "cells": [
        "012",
        "Another Title",
        "Another Description Sample Text",
        "Another Excerpt Demo Text",
        "2019-11-29T14:02:39.311Z"
      ]
    }
  ]
});
formatter.scenario({
  "name": "Post A New Book Object",
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
  "name": "011 And Feature Title Book Obeject Properties Are Available For Example",
  "keyword": "Given "
});
formatter.match({
  "location": "CreatingNewPostRequest.checkingBookIDAndTitle(int,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "All Object Properties Are 011 Feature Title Feature Description Sample Text Feature Excerpt Demo Text 2019-11-25T14:02:37.311Z Ready To Use For Making This POST API Request",
  "keyword": "When "
});
formatter.match({
  "location": "CreatingNewPostRequest.makingAPIRequestReady(int,String,String,String,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "I Verify REST API Post Request Status Code \u003cCode\u003e",
  "keyword": "Then "
});
formatter.match({
  "location": "CreatingNewPostRequest.verifyingCode(String)"
});
formatter.result({
  "status": "passed"
});
formatter.scenario({
  "name": "Post A New Book Object",
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
  "name": "012 And Another Title Book Obeject Properties Are Available For Example",
  "keyword": "Given "
});
formatter.match({
  "location": "CreatingNewPostRequest.checkingBookIDAndTitle(int,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "All Object Properties Are 012 Another Title Another Description Sample Text Another Excerpt Demo Text 2019-11-29T14:02:39.311Z Ready To Use For Making This POST API Request",
  "keyword": "When "
});
formatter.match({
  "location": "CreatingNewPostRequest.makingAPIRequestReady(int,String,String,String,String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "I Verify REST API Post Request Status Code \u003cCode\u003e",
  "keyword": "Then "
});
formatter.match({
  "location": "CreatingNewPostRequest.verifyingCode(String)"
});
formatter.result({
  "status": "passed"
});
});