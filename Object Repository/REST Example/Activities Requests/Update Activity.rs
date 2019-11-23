<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update A Specific Activity.</description>
   <name>Update Activity</name>
   <tag></tag>
   <elementGuidId>8444d198-dda8-43bd-9bce-06fff0adc5d3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ID\&quot;: \&quot;${activity_ID}\&quot;,\n  \&quot;Title\&quot;: \&quot;${activity_Title}\&quot;,\n  \&quot;DueDate\&quot;: \&quot;2019-11-22T14:21:59.736Z\&quot;,\n  \&quot;Completed\&quot;: true\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://fakerestapi.azurewebsites.net/api/Activities/${activity_ID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>23</defaultValue>
      <description></description>
      <id>1e3e7651-ef26-4036-a250-86341156f1d6</id>
      <masked>false</masked>
      <name>activity_ID</name>
   </variables>
   <variables>
      <defaultValue>'Updated Activity'</defaultValue>
      <description></description>
      <id>6c14af5e-bfd3-4880-b44d-2df6c3f73b92</id>
      <masked>false</masked>
      <name>activity_Title</name>
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

WS.verifyElementPropertyValue(response, 'Title', &quot;Updated Activity&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
