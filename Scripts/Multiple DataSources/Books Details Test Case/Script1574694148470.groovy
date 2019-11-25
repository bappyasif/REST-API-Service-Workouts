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

WS.sendRequest(findTestObject('REST Example/Books Requests/Get Books Details Request'))

WS.sendRequest(findTestObject('REST Example/Books Requests/Get Books Details Request'))

WS.sendRequest(findTestObject('REST Example/Books Requests/Post New Book Request', [('book_id') : 11, ('book_title') : 'Some Title'
            , ('book_description') : 'Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. '
            , ('book_excerpt') : 'Book Excerpt text Sample. Book Excerpt text Sample. ', ('books_published') : '']))

WS.sendRequest(findTestObject('REST Example/Books Requests/Delete Specefic Book Request'))

WS.sendRequest(findTestObject('REST Example/Books Requests/Update Specefic Book Request', [('book_id') : 0, ('book_title') : 'Updated Title'
            , ('book_description') : 'Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. '
            , ('book_excerpt') : 'Updated Book Excerpt text. Updated Book Excerpt text. Updated Book Excerpt text. Updated Book Excerpt text. '
            , ('book_published') : '"2019-11-25T14:02:37.311Z"']))

