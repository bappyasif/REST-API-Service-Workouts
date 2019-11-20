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

println (post_id)

println (post_title)

println (updated_title)

postResponse = WS.sendRequest(findTestObject('Json Placeholder/Create Post', [('user_id') : post_id, ('post_title') : post_title]))

def jsonRequest = new groovy.json.JsonSlurper();

def postContent = jsonRequest.parseText(postResponse.responseBodyContent);

def extractId = postContent.userId

def extractTitle = postContent.title 

updatedResponse = WS.sendRequest(findTestObject('Json Placeholder/Update Post', [('user_id') : post_id, ('post_title') : updated_title]))

def updateContent = jsonRequest.parseText(updatedResponse.getResponseBodyContent());

def extractUpdatedPostTitle = updateContent.title

//assert post_id == evaluate(extractId)

//assert Integer.parseInt(extractId) == post_id

// When Running From Test Suites It Runs Fine But From Test Case Scenerio Choose From Either Two Of The Earlier Statement. 
assert extractId == post_id

assert extractTitle == post_title

assert extractUpdatedPostTitle == updated_title

post_id = extractId

post_title = extractTitle

updated_title = extractUpdatedPostTitle

println (post_id)

println (post_title)

println (updated_title)