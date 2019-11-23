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

listActivities = WS.sendRequest(findTestObject('REST Example/Activities Requests/Get Activities'))

def jsonResponse = new groovy.json.JsonSlurper()

def contentResponse = jsonResponse.parseText(listActivities.responseBodyContent)

def extract_ID = contentResponse[1].ID

def extract_Boolean = contentResponse[1].Completed

println(extract_ID)

println(extract_Boolean)

singleActivity = WS.sendRequest(findTestObject('REST Example/Activities Requests/Spcefic Activity', [('activity_ID') : extract_ID]))

def singleResponse = jsonResponse.parseText(singleActivity.getResponseBodyContent())

def single_ID = singleResponse.ID

def single_Boolean = singleResponse.Completed

assert extract_Boolean == single_Boolean

assert extract_ID == single_ID

updateActivity = WS.sendRequest(findTestObject('REST Example/Activities Requests/Update Activity', [('activity_ID') : single_ID, ('activity_Title') : 'Updated Activity']))

def updateResponse = jsonResponse.parseText(updateActivity.responseBodyContent)

def updated_ID = updateResponse.ID

def updated_Title = updateResponse.Title

ACTIVITY_ID = updated_ID

ACTIVITY_TITLE = updated_Title

assert ACTIVITY_ID == updated_ID

assert ACTIVITY_TITLE == updated_Title

deleteActivity = WS.sendRequest(findTestObject('REST Example/Activities Requests/Delete Post', [('activity_ID') : ACTIVITY_ID]))

//postActivity = WS.sendRequest(findTestObject('REST Example/Activities Requests/Post Activity', [('activity_ID') : 23, ('activity_Title') : 'Created Title']))
postActivity = WS.sendRequest(findTestObject('REST Example/Activities Requests/Post Activity', [('activity_ID') : single_ID, ('activity_Title') : 'Created Title']))

def postResponse = jsonResponse.parseText(postActivity.responseBodyContent)

def created_ID = postResponse.ID

def creted_Title = postResponse.Title

println(created_ID)

println(creted_Title)

assert created_ID == single_ID

assert creted_Title == 'Created Title'

WS.sendRequestAndVerify(findTestObject('REST Example/Activities Requests/Update Activity', [('activity_ID') : 23, ('activity_Title') : 'Updated Activity']))

