<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Spcefic Activity</description>
   <name>Spcefic Activity</name>
   <tag></tag>
   <elementGuidId>30e1c986-5747-4b50-9931-8415dbac3796</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://fakerestapi.azurewebsites.net/api/Activities/${activity_ID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>e33d35e1-e89a-40a0-aa34-4bdc78949875</id>
      <masked>false</masked>
      <name>activity_ID</name>
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

WS.verifyElementPropertyValue(response, 'Title', &quot;Activity 2&quot;)
WS.verifyElementPropertyValue(response, 'Completed', true)


WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
