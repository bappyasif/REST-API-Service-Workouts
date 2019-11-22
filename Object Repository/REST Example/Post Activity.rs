<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Post Activity</description>
   <name>Post Activity</name>
   <tag></tag>
   <elementGuidId>57041fc2-f427-4c91-9487-9fe05cd08b35</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ID\&quot;: \&quot;${activity_ID}\&quot;,\n  \&quot;Title\&quot;: \&quot;${activity_Title}\&quot;,\n  \&quot;DueDate\&quot;: \&quot;2019-11-22T14:21:59.735Z\&quot;,\n  \&quot;Completed\&quot;: true\n}&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://fakerestapi.azurewebsites.net/api/Activities</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>23</defaultValue>
      <description></description>
      <id>9bda623b-a98b-4d38-9bad-1b47c924be1a</id>
      <masked>false</masked>
      <name>activity_ID</name>
   </variables>
   <variables>
      <defaultValue>'Created Title'</defaultValue>
      <description></description>
      <id>e9c0963e-a9db-4fae-ae1c-e00086dbe60c</id>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
