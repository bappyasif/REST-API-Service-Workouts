#Author: your.email@your.domain.com
#Keywords Summary :
#Feature: List of scenarios.
#Scenario: Business rule through list of steps with arguments.
#Given: Some precondition step
#When: Some key actions
#Then: To observe outcomes or validation
#And,But: To enumerate more Given,When,Then steps
#Scenario Outline: List of steps for data-driven as an Examples and <placeholder>
#Examples: Container for s table
#Background: List of steps run before each of the scenarios
#""" (Doc Strings)
#| (Data Tables)
#@ (Tags/Labels):To group Scenarios
#<> (placeholder)
#""
## (Comments)
#Sample Feature Definition Template
@tag
Feature: REST API Requests Example
  I want to use this template for my feature file

  @tag1
  Scenario Outline: Post A New Book Object
    Given <Book_ID> And <Book_Title> Book Obeject Properties Are Available For Example
    When All Object Properties Are <Book_ID> <Book_Title> <Book_Description> <Book_Excerpt> <Published_Date> Ready To Use For Making This POST API Request
    Then I Verify REST API Post Request Status Code <Code>

    Examples: 
      | Book_ID | Book_Title | Book_Description | Book_Excerpt | Published_Date |
      | 011   |  Feature Title | Feature Description Sample Text | Feature Excerpt Demo Text | 2019-11-25T14:02:37.311Z |
      | 012   |  Another Title | Another Description Sample Text | Another Excerpt Demo Text | 2019-11-29T14:02:39.311Z |
