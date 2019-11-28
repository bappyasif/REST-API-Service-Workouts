<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get Specefic Book Request</description>
   <name>Get Specefic Book Request</name>
   <tag></tag>
   <elementGuidId>83d0c890-6a78-4dfe-912f-4a0a2abc95d2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://fakerestapi.azurewebsites.net/api/Books/${book_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>00247188-8203-4894-bf23-aa379ee267ca</id>
      <masked>false</masked>
      <name>book_id</name>
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

assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))

WS.verifyElementPropertyValue(response, 'PageCount', 200)

//assertThat(response.getResponseText()).contains('Katalon Test Project')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
