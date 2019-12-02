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

newEmployee = WS.sendRequest(findTestObject('Dummy Example/Create New Employee Record', [('employee_name') : 'Bappy', ('salary_amount') : 99999999
            , ('employee_age') : 26]))

def jsonResponse = new groovy.json.JsonSlurper()

def nEmpResponse = jsonResponse.parseText(newEmployee.getResponseBodyContent())

def extract_nEmpID = nEmpResponse.id

println extract_nEmpID

//def trnasfromedInteger = Integer.valueOf(extract_nEmpID)
def trnasfromedInteger = evaluate(extract_nEmpID)

println trnasfromedInteger

def extract_nEmpName = nEmpResponse.name

println extract_nEmpName

def extract_nEmpSalary = nEmpResponse.salary

println extract_nEmpSalary

def extract_nEmpAge = nEmpResponse.age

println extract_nEmpAge

employeeList = WS.sendRequest(findTestObject('Dummy Example/All Employees'))

def employeeResponse = jsonResponse.parseText(employeeList.getResponseBodyContent())

println (employeeList.getStatusCode())

println employeeResponse

//def extract_ID = employeeResponse[trnasfromedInteger].id
//
//println extract_ID
//
//def extract_Name = employeeResponse[trnasfromedInteger].employee_name
//
//println extract_Name
//
//def extract_Salary = employeeResponse[trnasfromedInteger].employee_salary
//
//println extract_Salary
//
//def extract_Age = employeeResponse[trnasfromedInteger].employee_age
//
//println extract_Age
//
//assert extract_nEmpID == extract_ID
//
//assert extract_nEmpName == extract_Name
//
//assert extract_nEmpSalary == extract_Salary
//
//assert extract_nEmpAge == extract_Age

updateEmployee = WS.sendRequest(findTestObject('Dummy Example/Update Employee', [('employee_name') : extract_nEmpName, ('salary_amount') : extract_nEmpSalary
            , ('employee_age') : extract_nEmpAge, ('employee_id') : extract_nEmpID]))

println (updateEmployee.getStatusCode())

def updateResponse = jsonResponse.parseText(updateEmployee.responseBodyContent)

def extract_uName = updateResponse.name

def extract_uSalary = updateResponse.salary

def extract_uAge = updateResponse.age

//assert extract_Age == extract_uAge
//
//assert extract_Salary == extract_uSalary
//
//assert extract_Name == extract_uName

deleteEmployee = WS.sendRequest(findTestObject('Dummy Example/Delete Employee', [('employee_id') : extract_nEmpID]))

println (deleteEmployee.getStatusCode())

employeeListRePopulated = WS.sendRequest(findTestObject('Dummy Example/All Employees'))

def elrpResponse = jsonResponse.parseText(employeeListRePopulated.responseBodyContent)

//def trnsformAgain = Integer.valueOf(extract_ID)
//
//println trnsformAgain

//def extract_fID = elrpResponse[trnsformAgain].id
//
//if(extract_fID == extract_nEmpID) {
//	println ("Something Fishy!!")
//} else {
//	println ("Seems Okay!!")
//}

println (employeeListRePopulated.getStatusCode())