import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

println (Book_ID)

println (Book_Title)

println (Book_Description)

println (Book_Excerpt)

println (Book_Published)

Date date = new Date()

String dateFormat = date.calendarDate;

println (dateFormat); 

Book_Published = dateFormat

//WS.sendRequest(findTestObject('REST Example/Books Requests/Get Books Details Request'))

//WS.sendRequest(findTestObject('REST Example/Books Requests/Get Books Details Request'))

createdResponse = WS.sendRequest(findTestObject('REST Example/Books Requests/Post New Book Request', [('book_id') : Book_ID, ('book_title') : Book_Title, ('book_description') : Book_Description, ('book_excerpt') : Book_Excerpt, ('books_published') : Book_Published]))

//WS.sendRequest(findTestObject('REST Example/Books Requests/Delete Specefic Book Request'))

def jsonResponse = new groovy.json.JsonSlurper()

def createdContent = jsonResponse.parseText(createdResponse.responseBodyContent)

def extract_BookID = createdContent.ID

def extract_BookTitle = createdContent.Title

def extract_BookDescription = createdContent.Description

def extract_BookExcerpt = createdContent.Excerpt

def extract_BookPDate = createdContent.PublishDate

//assert Book_ID == extract_BookID
//assert Book_ID == evaluate(extract_BookID)

assert Book_Title == extract_BookTitle

assert Book_Description == extract_BookDescription

assert Book_Excerpt == extract_BookExcerpt

//assert Book_Published == extract_BookPDate

println (Updated_Title)

println (Updated_Description)

println (Updated_Excerpt)

updatedResponse = WS.sendRequest(findTestObject('REST Example/Books Requests/Update Specefic Book Request', [('book_id') : Book_ID, ('book_title') : Updated_Title
            , ('book_description') : Updated_Description, ('book_excerpt') : Updated_Excerpt, ('book_published') : Book_Published]))

def updatedContent = jsonResponse.parseText(updatedResponse.responseBodyContent)

def extract_updatedTitle = updatedContent.Title

def extract_updatedDescription = updatedContent.Description

def extract_updatedExcerpt = updatedContent.Excerpt

assert Updated_Title == extract_updatedTitle

assert Updated_Description == extract_updatedDescription

assert Updated_Excerpt == extract_updatedExcerpt 
