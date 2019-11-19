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

postsList = WS.sendRequest(findTestObject('Json Placeholder/List Posts'))

def jsonSlurper = new groovy.json.JsonSlurper()

//def jsonResponse = jsonSlurper.parseText(postsList.getResponseText())
def postslistsResponseResult = jsonSlurper.parseText(postsList.responseBodyContent)

def extractPost = postslistsResponseResult[25].id

println(extractPost)

//println (postslistsResponseResult)
//println (jsonResponse)
newPost = WS.sendRequest(findTestObject('Json Placeholder/Create Post', [('user_id') : extractPost, ('post_title') : 'Newly Posted']))

//def newwlyCreated = jsonSlurper.parseText(newPost.getResponseText())
def newwlyCreated = jsonSlurper.parseText(newPost.responseBodyContent)

def extractTitle = newwlyCreated.title

println(extractTitle)

//println(newwlyCreated)
editPost = WS.sendRequest(findTestObject('Json Placeholder/Update Post', [('user_id') : extractPost, ('post_title') : 'Updated Recently']))

def updatedPost = jsonSlurper.parseText(editPost.getResponseText())

def updatedTitle = updatedPost.title

println(updatedTitle)

WS.sendRequestAndVerify(findTestObject('Json Placeholder/List Posts'))

