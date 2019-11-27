import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.InternalData
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

//BOOK_ID =

//println (Book_ID); 

//i = 0;

//for (i = 0; i<Book_ID.length(); i++) {
//	for (j = i; j< Book_ID.length() - 1; j++) {
//		println (Book_ID[i, j][j]);
//	}
//}


//TestData Books_ID = findTestData("Data Files/Multiple Sources/Books Details Test Data")
//
//println (Books_ID)
//
//int rowsCount = Books_ID.getRowNumbers()
//
//println (rowsCount)

//for ( i = 0; i < rowsCount ; i++ ) {
//	for ( j = i; j< 5; j++) {
//		//println ([i],[j])
//		println (Books_ID.getObjectValue([i, j]))
//	}
//}

//InternalData ID_Numbers = findTestData("Data Files/Multiple Sources/Books Details Test Data")

//println Books_ID.getAllData();
//
//println Books_ID.getColumnNumbers(); 
//
//for (i = 1; i< rowsCount; i++) {
//	for(j = i; j<Books_ID.getColumnNumbers(); j++) {
//		//println Books_ID.getValue([i], [j])
//		println Books_ID.getObjectValue(i,j)
//	}
//}
//
////println Book_ID.getValue(1,2)
//
//println Books_ID.getValue(1,2)

//WS.sendRequest(findTestObject('REST Example/Books Requests/Get Specefic Book Request', [('book_id') : Books_ID.getValue(1, 1)]))
//
//WS.sendRequest(findTestObject('REST Example/Books Requests/Delete Specefic Book Request', [('book_id') : Books_ID.getValue(1, 2)]))


TestData Books_ID = findTestData("Data Files/Multiple Sources/Books Details Test Data")

int rowsCount = Books_ID.getRowNumbers()

println (rowsCount)

for(i=1; i<=rowsCount; i++) {
	
	WS.sendRequest(findTestObject('REST Example/Books Requests/Get Specefic Book Request', [('book_id') : Books_ID.getValue(1, i)]))
	
	WS.sendRequest(findTestObject('REST Example/Books Requests/Delete Specefic Book Request', [('book_id') : Books_ID.getValue(1, i)]))
}