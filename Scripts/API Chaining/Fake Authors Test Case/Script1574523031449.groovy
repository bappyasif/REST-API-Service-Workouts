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

authorsList = WS.sendRequest(findTestObject('REST Example/Authors Requests/Get Authors Details Request'))

def jsonResponse = new groovy.json.JsonSlurper()

def authorsContent = jsonResponse.parseText(authorsList.getResponseBodyContent())

def extract_bookID = authorsContent[1].IDBook

println(extract_bookID)

def extract_authorID = authorsContent[1].ID

def extract_aFname = authorsContent[1].FirstName

def extract_aLname = authorsContent[1].LastName

println(extract_authorID)

println(extract_aFname)

println(extract_aLname)

booksList = WS.sendRequest(findTestObject('REST Example/Authors Requests/Get Authors Books Request', [('book_id') : extract_bookID]))

def booksContent = jsonResponse.parseText(booksList.responseBodyContent)

def check_bookID = booksContent[extract_bookID].IDBook

//assert extract_bookID == evaluate("check_bookID")
println(check_bookID)

assert check_bookID == extract_bookID

postDetails = WS.sendRequest(findTestObject('REST Example/Authors Requests/Post Authors Details Request', [('book_id') : check_bookID
            , ('author_id') : extract_authorID, ('first_name') : extract_aFname, ('last_name') : extract_aLname]))

def postContents = jsonResponse.parseText(postDetails.getResponseBodyContent())

def extract_postedID = postContents.ID

def extract_postedBookID = postContents.IDBook

def extract_postedAFName = postContents.FirstName

def extract_postedALName = postContents.LastName

//println(extract_postedID)

println(extract_postedBookID)

println(extract_postedAFName)

println(extract_postedALName)

assert extract_postedID == extract_authorID

assert extract_postedAFName == extract_aFname

assert extract_postedALName == extract_aLname

GlobalVariable.authorID = extract_postedID

deleteStatus = WS.sendRequest(findTestObject('REST Example/Authors Requests/Delete Author Details Request', [('author_id') : GlobalVariable.authorID]))

//assertThat(deleteStatus.getStatusCode()).isEqualTo(200)
WS.verifyResponseStatusCode(deleteStatus, 200)

updateDetails = WS.sendRequest(findTestObject('REST Example/Authors Requests/Update Authors Details Request', [('author_id') : extract_postedID
            , ('book_id') : extract_postedBookID, ('updated_fName') : extract_postedAFName, ('updated_lName') : extract_postedALName]))

def updatedContents = jsonResponse.parseText(updateDetails.responseBodyContent)

def extract_updatedAuthorID = updatedContents.ID

def extract_updatedBookID = updatedContents.IDBook

def extract_updatedAuthorFName = updatedContents.FirstName

def extract_updatedAuthorLName = updatedContents.LastName

println (extract_updatedAuthorID)

println (extract_updatedBookID)

println (extract_updatedAuthorFName)

println (extract_updatedAuthorLName)

assert extract_updatedAuthorID == extract_authorID

assert extract_updatedBookID == extract_bookID

assert extract_updatedAuthorFName == extract_aFname

assert extract_updatedAuthorLName == extract_aLname


