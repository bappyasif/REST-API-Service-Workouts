<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get a single employee data</description>
   <name>Specefic Employee</name>
   <tag></tag>
   <elementGuidId>dd54f150-f86d-4e8a-a55f-053ee2494811</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/employee/2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>d689a6b4-c3f4-488b-a938-3d107543e307</id>
      <masked>false</masked>
      <name>employee_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


//WS.verifyElementPropertyValue(response, '[0].id', '1')


//WS.verifyElementPropertyValue(response, '[0].id', '1')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
