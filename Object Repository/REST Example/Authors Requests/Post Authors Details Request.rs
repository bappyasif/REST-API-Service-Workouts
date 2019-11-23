<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Post Authors Details Request</description>
   <name>Post Authors Details Request</name>
   <tag></tag>
   <elementGuidId>7c9d79b0-b904-4e4e-8a7d-64a861736d54</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ID\&quot;: \&quot;${author_id}\&quot;,\n  \&quot;IDBook\&quot;: \&quot;${book_id}\&quot;,\n  \&quot;FirstName\&quot;: \&quot;${first_name}\&quot;,\n  \&quot;LastName\&quot;: \&quot;${last_name}\&quot;\n}&quot;,
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
   <restUrl>http://fakerestapi.azurewebsites.net/api/Authors</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>3e88ac12-657a-4a74-b021-9ed19cc7d4e2</id>
      <masked>false</masked>
      <name>book_id</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>8203ce34-cb6c-437d-990a-626c9b22cddc</id>
      <masked>false</masked>
      <name>author_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f8277d5b-9180-4f5c-94f3-961b36c4d7d8</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4d1593c4-23e0-4350-a2e9-3b9490f6ab6b</id>
      <masked>false</masked>
      <name>last_name</name>
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
